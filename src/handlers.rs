use crate::models::{Rr, R};
use actix_web::http::header::HeaderMap;
use actix_web::{HttpRequest, HttpResponse};

const HEADER_CF_CONNECTING_IP: &str = "cf-connecting-ip";
const HEADER_CF_IP_COUNTRY: &str = "cf-ipcountry";
const HEADER_X_FORWARDED_FOR: &str = "x-forwarded-for";

pub struct Handler;

impl Handler {
    pub async fn ip(req: HttpRequest) -> Result<HttpResponse, actix_web::Error> {
        let headers: &HeaderMap = req.headers();

        let ip: Option<String> = Self::get_ip_from_header(&headers, HEADER_CF_CONNECTING_IP)
            .or_else(|| Self::get_ip_from_header(&headers, HEADER_CF_IP_COUNTRY))
            .or_else(|| {
                Self::get_ip_from_header(&headers, HEADER_X_FORWARDED_FOR)
                    .and_then(|ip: String| ip.split(',').next().map(|s: &str| s.to_string()))
            });

        let response: R = match ip {
            Some(ip) => R {
                status: true,
                message: "success".to_string(),
                result: Some(Rr { ip }),
            },
            None => R {
                status: false,
                message: "failed".to_string(),
                result: None,
            },
        };

        Ok(HttpResponse::Ok().json(response))
    }
    pub async fn health() -> Result<HttpResponse, actix_web::Error> {
        Ok(HttpResponse::Ok().json(R {
            status: true,
            message: "ok".to_string(),
            result: None,
        }))
    }
    fn get_ip_from_header(headers: &HeaderMap, header_name: &str) -> Option<String> {
        headers
            .get(header_name)
            .and_then(|value: &actix_web::http::header::HeaderValue| {
                value.to_str().ok().map(|s: &str| s.to_string())
            })
    }
}
