// ==============================================================================
//
// Inspired by the Actix web static files Angular router example:
// https://github.com/kilork/actix-web-static-files-example-angular-router/
//
// ==============================================================================

use actix_web_static_files;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));
