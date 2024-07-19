// webpage_hit_counter/src/hello_mod.rs

//! All the real code is inside modules in separate files.
//!
//! This doc-comments will be compiled into the `docs`.

/// extract path info from "/webpage_hit_counter/get_svg_image/{webpage_id}.svg" url
/// {webpage_id} - deserializes to a i32
pub async fn get_svg_image(db_pool: actix_web::web::Data<deadpool_postgres::Pool>, path: actix_web::web::Path<i32>) -> impl actix_web::Responder {
    let webpage_id = path.into_inner();

    println!("webpage_hit_counter/get_svg_image/{webpage_id}.svg");
    let hit_count = crate::postgres_mod::select_count(db_pool, webpage_id).await.unwrap();

    actix_web::HttpResponse::Ok()
        .append_header(actix_web::http::header::ContentType(mime::IMAGE_SVG))
        .append_header(actix_web::http::header::CacheControl(vec![
            actix_web::http::header::CacheDirective::NoCache,
            actix_web::http::header::CacheDirective::MaxAge(0u32),
            actix_web::http::header::CacheDirective::NoStore,
            actix_web::http::header::CacheDirective::SMaxAge(0u32),
            actix_web::http::header::CacheDirective::ProxyRevalidate,
        ]))
        .append_header(("Pragma", "no-cache"))
        .append_header(("Expires", "-1"))
        .body(format!(
            r##"
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
    "##
        ))
}

#[cfg(test)]
mod test {}
