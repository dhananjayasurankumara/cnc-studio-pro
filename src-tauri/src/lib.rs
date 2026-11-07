use std::{io::{Read,Write},sync::{Arc,Mutex,atomic::{AtomicBool,Ordering}},thread,time::Duration};
use tauri::{Emitter,Manager};

struct SerialState { port: Mutex<Option<Arc<Mutex<Box<dyn serialport::SerialPort>>>>>, running: Mutex<Option<Arc<AtomicBool>>> }

#[tauri::command]
fn list_ports() -> Result<Vec<String>,String> {
    serialport::available_ports().map(|p| p.into_iter().map(|p| p.port_name).collect()).map_err(|e| e.to_string())
}

#[tauri::command]
fn connect(app:tauri::AppHandle,state:tauri::State<'_,SerialState>,name:String,baud:u32)->Result<(),String>{
    if state.port.lock().map_err(|e|e.to_string())?.is_some(){return Err("Disconnect first".into());}
    let port=serialport::new(&name,baud).timeout(Duration::from_millis(80)).open().map_err(|e|e.to_string())?;
    let shared=Arc::new(Mutex::new(port));
    let running=Arc::new(AtomicBool::new(true));
    *state.port.lock().map_err(|e|e.to_string())?=Some(shared.clone());
    *state.running.lock().map_err(|e|e.to_string())?=Some(running.clone());
    thread::spawn(move || {
        let mut pending=Vec::<u8>::new();
        let mut bytes=[0u8;256];
        while running.load(Ordering::SeqCst){
            let result=match shared.lock(){Ok(mut p)=>p.read(&mut bytes),Err(_)=>break};
            match result {
                Ok(n)=>for b in &bytes[..n] {
                    if *b==b'\n' || *b==b'\r' {
                        if !pending.is_empty(){let line=String::from_utf8_lossy(&pending).trim().to_string();if !line.is_empty(){let _=app.emit("grbl-line",line);}pending.clear();}
                    }else if pending.len()<4096 {pending.push(*b);}else{pending.clear();}
                },
                Err(e) if e.kind()==std::io::ErrorKind::TimedOut=>{},
                Err(e)=>{let _=app.emit("serial-error",e.to_string());break;}
            }
        }
        let _=app.emit("serial-closed",());
    });
    Ok(())
}

#[tauri::command]
fn disconnect(state:tauri::State<'_,SerialState>)->Result<(),String>{
    if let Some(flag)=state.running.lock().map_err(|e|e.to_string())?.take(){flag.store(false,Ordering::SeqCst);}
    state.port.lock().map_err(|e|e.to_string())?.take();Ok(())
}

#[tauri::command]
fn send(state:tauri::State<'_,SerialState>,data:Vec<u8>)->Result<(),String>{
    let port=state.port.lock().map_err(|e|e.to_string())?.clone().ok_or("Not connected")?;
    let mut guard=port.lock().map_err(|e|e.to_string())?;
    guard.write_all(&data).map_err(|e|e.to_string())
}

#[tauri::command]
fn reset_board(state:tauri::State<'_,SerialState>)->Result<(),String>{
    let port=state.port.lock().map_err(|e|e.to_string())?.clone().ok_or("Not connected")?;
    let mut guard=port.lock().map_err(|e|e.to_string())?;
    guard.write_all(&[0x18]).map_err(|e|e.to_string())
}

pub fn run(){
    tauri::Builder::default()
        .manage(SerialState{port:Mutex::new(None),running:Mutex::new(None)})
        .invoke_handler(tauri::generate_handler![list_ports,connect,disconnect,send,reset_board])
        .run(tauri::generate_context!()).expect("error running CNC Studio Pro");
}
