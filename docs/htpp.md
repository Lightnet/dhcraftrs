


```rs
let response = match reqwest::blocking::get(&request_url) {
    // Unwraps the response value from the Result
    Ok(r) => r,

    // Unwraps the error from the Result and terminates main()
    Err(err) => {
        println!("Request failed: {}", err.to_string());
        return;
    },
};
```