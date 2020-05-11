# TestAx

[![Build Status](https://github.com/fbucek/testax/workflows/build/badge.svg)](https://github.com/fbucek/testax/actions)
[![Documentation](https://docs.rs/testax/badge.svg)](https://docs.rs/testax)
[![crates.io](https://meritbadge.herokuapp.com/testax)](https://crates.io/crates/testax)

Simple crate for testing basic actix GET/POST/UPDATE/DELETE

```toml
[dev-dependencies]
testax = "0.1"
```

## TODO

- [x] GET 
- [x] POST
- [ ] UPDATE
- [ ] DELETE
- [ ] Universal??
 
## Minimal example

```rust
async fn index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

#[actix_rt::test]
async fn test_minimal() {
    let mut app = test::init_service(App::new().service(index)).await;

    let resp = get(&mut app, "/32/Filip").await;
    assert_eq!(resp.status.as_u16(), 200);
    assert_eq!(resp.body, "Hello Filip! id:32");
}
```

