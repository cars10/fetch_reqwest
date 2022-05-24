# fetch_reqwest

**Warning**: This library was specifically written for [elasticvue](https://github.com/cars10/elasticvue) - it might not fit your usecase.

It provides a very simple (and very incomplete) wrapper for [reqwest](https://github.com/seanmonstar/reqwest) that mimics the javascript fetch API.


## Usecase

I have an application written in vue.js (that heavily uses javascript fetch) that i want to package as a desktop app with [tauri](https://tauri.studio).
In my specific usecase i want to completely ignore ssl & certificate errors, this is something that neither the native javascript fetch nor tauri's http client can do - but you can "manually" do it with reqwest.
So we basically execute every network request from the rust process inside tauri and only return the result to the javascript application.

To use this library with tauri you have to replace every call to `fetch` in your application and instead invoke the `fetch` method exposed by fetch_reqwest. 

## API

Right now only the following options are supported:

### Request

```javascript
const options = {
    method: 'GET', // use any valid HTTP method
    headers: {}, // {Authorization: 'foo-bar'}
    body: '' // whatever you like
}
```

### Response

```javascript
response.ok // true || false
response.status // 200, 500, etc
response.status_text // OK, Created, etc
response.headers // response.headers.get('Authorization')
response.text().then(t => console.log(t)) // response body as str, e.g. "foo bar"
response.json().then(j => console.log(j)) // response body as json, e.g. {foo: "bar"}
```


## Example

### Preface

1. Make sure that you did [setup tauri](https://tauri.studio/docs/getting-started/prerequisites)
2. Install tauri in your app
3. Make sure that `@tauri-apps/cli` and `@tauri-apps/api` are included in your `package.json`

### Setup

1. Add `fetch_reqwest` as a dependency to your tauri project under `src-tauri`

```toml
[dependencies]
fetch_reqwest = { git = "https://github.com/cars10/fetch_reqwest" }
```

2. Add a tauri command that calls `fetch_reqwest` in your `src-tauri/src/main.rs`. MAKE SURE TO ALSO ADD THE CALL TO `.invoke_handler` in your `main()`

```rust
use fetch_reqwest::{FetchOptions, FetchResponseResult};

#[tauri::command]
async fn fetch_reqwest(resource: String, init: Option<FetchOptions>) -> FetchResponseResult {
    fetch_reqwest::fetch(resource, init).await
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fetch_reqwest]) // ADD THIS LINE TO USE THE COMMAND!!!
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

3. Add this JS (in the future this will be a seperate npm package)

```javascript
import { invoke } from '@tauri-apps/api/tauri'

export const fetchReqwest = (resource, init) => {
  return invoke('fetch_reqwest', { resource, init }).then(r => (new FetchReqwestResponse(r)))
}

class FetchReqwestResponseHeaders {
  constructor (headers) {
    this.headers = headers
  }

  get (header) {
    return this.headers[header]
  }
}

class FetchReqwestResponse {
  constructor (response) {
    this.headers = new FetchReqwestResponseHeaders(response.headers)
    this.ok = response.ok
    this.status = response.status
    this.statusText = response.status_text
    this.body = response.text
  }

  text () {
    return Promise.resolve(this.body)
  }

  json () {
    try {
      return Promise.resolve(JSON.parse(this.body))
    } catch (e) {
      return Promise.reject(e)
    }
  }
}

```

4. Use `fetchReqwest` just as you would use fetch:

```javascript
fetchReqwest("https://example.com", {
    method: "GET"
})
```