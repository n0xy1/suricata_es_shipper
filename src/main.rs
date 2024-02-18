use std::{io::{SeekFrom, Seek, BufReader, BufRead, Read}, fs::{File, self}, thread, time::Duration, sync::mpsc, process::exit};
use elasticsearch::{Elasticsearch, Error, http::{transport::{TransportBuilder, SingleNodeConnectionPool}, Url}, IndexParts, params::Refresh, auth::Credentials, indices::{IndicesExistsParts, IndicesCreateParts}};
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
struct Config {
    index: String,
    file_to_monitor: String,
    api_id: String,
    api_key: String,
    es_url: String,
}

fn read_config() -> Result<Config, Box<dyn std::error::Error>> {
    let mut file = fs::File::open("config.yaml")?;
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {
            match serde_yaml::from_str::<Config>(&contents) {
                Ok(config) => Ok(config),
                Err(e) => {
                    println!("Error parsing YAML: {:?}, using defaults.", e);
                    Ok(Config{index: "suricata".to_string(), file_to_monitor: "/var/log/suricata/eve.json".to_string(), api_id: "placeholder".to_string(), api_key: "placeholder".to_string(), es_url: "https://someurl.com".to_string()})
                }
            }
        },
        Err(e) => {
            println!("Error reading file: {:?}", e);
            Err(Box::new(e))
        },
    }
    
}

async fn index_data(client: &Elasticsearch, data: serde_json::Value, index: &String ) -> Result<(),Error> {
    let index_resp = client.index(IndexParts::Index(index))
    .body(data)
    .refresh(Refresh::WaitFor)
    .send()
    .await?;

    let json: Value = index_resp.json().await?;

    if json["errors"].as_bool().is_some() {
        let failed: Vec<&Value> = json["items"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|v| !v["error"].is_null())
            .collect();

        // TODO: retry failures
        println!("Errors whilst indexing. Failures: {}", failed.len());
    }
    Ok(())
}


fn monitor_log(path: String, tx: mpsc::Sender<String>){
    let mut file = File::open(path).expect("File not found");
    let mut position = file.seek(SeekFrom::End(0)).unwrap();

    loop {
        let new_position = file.seek(SeekFrom::End(0)).unwrap();
        if new_position != position {
            file.seek(SeekFrom::Start(position)).unwrap();
            let mut buffer = BufReader::new(&file);
            let mut line = String::new();
            while buffer.read_line(&mut line).unwrap() > 0{
                tx.send(line.clone()).unwrap();
                line.clear();
            }
            position = new_position;
        }
        thread::sleep(Duration::from_secs(5));
    }
}


#[tokio::main]
async fn main() -> Result<(),Error> {

    // read config
    let app_data = match read_config() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to read config: {}", e);
            exit(-1);
        }
    };


    // build elasticsearch client.
    let creds = Credentials::ApiKey(app_data.api_id, app_data.api_key);
    let conn_pool = SingleNodeConnectionPool::new(Url::parse(&app_data.es_url).unwrap());
    let builder = TransportBuilder::new(conn_pool).auth(creds).build()?;
    let client = Elasticsearch::new(builder);

    //check index.
    let exists = client.indices().exists(IndicesExistsParts::Index(&[&app_data.index]))
        .send()
        .await?;

    if exists.status_code() != 200 {
        let create_index_resp: elasticsearch::http::response::Response = client.indices().create(IndicesCreateParts::Index(&app_data.index))
        .send()
        .await?;
        if create_index_resp.status_code() != 200{
            panic!("Failed to create index.");
        }
        println!("Index created: {:?}", create_index_resp.status_code());
    }else{
        println!("Client ready, index exists: {:?}", exists.status_code());
    }

    let (tx,rx) = mpsc::channel();
    
    let file_monitor_thread = thread::spawn(move || {
        monitor_log(app_data.file_to_monitor, tx);
    });

    for received in rx{
        // Parse the string of data into serde_json::Value
        match serde_json::from_str(&received){
            Ok(v) => {
                // if it parses correctly, post it straight into the index.
                index_data(&client, v, &app_data.index).await?;
            },
            Err(e) => println!{"Error: {:?}", e}
        };
    }

    file_monitor_thread.join().unwrap();
    Ok(())
}
