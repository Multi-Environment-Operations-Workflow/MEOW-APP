#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use local_ip_address::local_ip;
use std::{net::Ipv4Addr, thread};
use tiny_http::{Method, Response, Server};
use tauri::{AppHandle, Emitter};

#[tauri::command]
pub fn start_server(app:AppHandle, offer: String) -> String {
    let ip = local_ip().unwrap();
    let port = 3005;

    let url = format!("http://{}:{}", ip, port);

    // Spawn server in new thread
    thread::spawn(move || {
        let server = Server::http((Ipv4Addr::UNSPECIFIED, port)).unwrap();
        println!("Server running on {} {}", local_ip().unwrap(), port);

        for mut req in server.incoming_requests() {
            if req.url() == "/init" {
                let response = Response::from_string(&offer);
                let _ = req.respond(response);
            }
            else if req.url() == "/answer" && *req.method() == Method::Post {
                let mut answer = String::new();
                req.as_reader().read_to_string(&mut answer).unwrap();
                let _ = req.respond(Response::from_string(""));
                
                app.emit("answer-ready", answer).unwrap();

            }
        }
    });
    println!("{}", url);
    return url;
}
