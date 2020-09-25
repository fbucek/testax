# Changes

## [0.3.0] - 2020-09-26

### Actix-web 3.0 update

* Updated to actix-web 3.0.x

## [0.2.0] - 2020-05-16

### Changed

* Breaking changes: methods returns `anyhow::Result<T>` due to unexpected panic when expecting `Authorizaion` header but is missing.

## [0.1.2] - 2020-05-16

### Changed

* Added  `call_service_res` which returns `Result<T>` instead of `T` in original `call_service` method

## [0.1.1] - 2020-05-10

### Changed

* Added  `post` test method which accept `struct` implementing `Serialize trait`  which returns `body` and `status`

## [0.1.0] - 2020-05-10

### Changed

* Initial commit with `get` test method which returns `body` and `status`
