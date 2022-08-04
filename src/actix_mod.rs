// webpage_hit_counter/src/hello_mod.rs

//! All the real code is inside modules in separate files.
//!
//! This doc-comments will be compiled into the `docs`.

use actix_web::{http::header, HttpResponse, Responder};

pub async fn get_image(db_pool: actix_web::web::Data<deadpool_postgres::Pool>) -> impl Responder {
    println!("get_image");
    let hit_count = crate::postgres_mod::select_count(db_pool).await.unwrap();

    HttpResponse::Ok()
    .append_header(header::ContentType(mime::IMAGE_SVG))
    .body(format!(r##"
<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" 
width="82" height="20">
 <linearGradient id="smooth" x2="0" y2="100%">
   <stop offset="0" stop-color="#bbb" stop-opacity=".1"/>
   <stop offset="1" stop-opacity=".1"/>
 </linearGradient>

 <mask id="round">
   <rect width="82" height="20" rx="3" ry="3" fill="#fff"/>
 </mask>

 <g mask="url(#round)">
   <rect width="42" height="20" fill="#555555"/>
   <rect x="42" width="52" height="20" fill="#79C83D"/>
   <rect width="82" height="20" fill="url(#smooth)"/>
 </g>

 <g fill="#fff" text-anchor="middle" font-family="Verdana,DejaVu Sans,Geneva,sans-serif" font-size="11"> 
   <text x="18" y="15" fill="#010101" fill-opacity=".3">Hits</text>
   <text x="18" y="14" fill="#fff">Hits</text>
   <text x="60" y="15" fill="#010101" fill-opacity=".3">{hit_count}</text>
   <text x="60" y="14" fill="#fff">{hit_count}</text>
 </g>
</svg>    
    "##))
}

#[cfg(test)]
mod test {}
