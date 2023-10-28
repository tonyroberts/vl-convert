// *************************************************************************
// * This file is generated by vl-convert-vendor/src/main.rs. Do not edit! *
// *************************************************************************
use deno_runtime::deno_core::anyhow::bail;
use deno_runtime::deno_core::error::AnyError;
use std::collections::HashMap;
use std::str::FromStr;

pub const SKYPACK_URL: &str = "https://cdn.skypack.dev";
pub const VEGA_PATH: &str =
    "/pin/vega@v5.25.0-r16knbfAAfBFDoUvoc7K/mode=imports,min/optimized/vega.js";
pub const VEGA_THEMES_PATH: &str =
    "/pin/vega-themes@v2.14.0-RvUmNETlVH2y3yQM1y36/mode=imports,min/optimized/vega-themes.js";
pub const VEGA_EMBED_PATH: &str =
    "/pin/vega-embed@v6.23.0-Fpmq39rehEH8HWtd6nzv/mode=imports,min/optimized/vega-embed.js";
pub const DEBOUNCE_PATH: &str = "/pin/lodash.debounce@v4.0.8-aOLIwnE2RethWPrEzTeR/mode=imports,min/optimized/lodash.debounce.js";

pub fn url_for_path(path: &str) -> String {
    format!("{}{}", SKYPACK_URL, path)
}

pub fn vega_url() -> String {
    url_for_path(VEGA_PATH)
}

pub fn vega_themes_url() -> String {
    url_for_path(VEGA_THEMES_PATH)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[allow(non_camel_case_types)]
pub enum VlVersion {
    v4_17,
    v5_8,
    v5_9,
    v5_10,
    v5_11,
    v5_12,
    v5_13,
    v5_14,
    v5_15,
    v5_16,
}

impl VlVersion {
    pub fn to_path(self) -> String {
        use VlVersion::*;
        let path = match self {
            v4_17 => "/pin/vega-lite@v4.17.0-ycT3UrEO81NWOPVKlbjt/mode=imports,min/optimized/vega-lite.js",
            v5_8 => "/pin/vega-lite@v5.8.0-4snbURNltT4se5LjMOKF/mode=imports,min/optimized/vega-lite.js",
            v5_9 => "/pin/vega-lite@v5.9.3-QyXScylQe0TTmb9DRCES/mode=imports,min/optimized/vega-lite.js",
            v5_10 => "/pin/vega-lite@v5.10.0-Vm0dgr6cpOyUiTjlPzt9/mode=imports,min/optimized/vega-lite.js",
            v5_11 => "/pin/vega-lite@v5.11.1-Q5Jhmb2acmWm03IObXvn/mode=imports,min/optimized/vega-lite.js",
            v5_12 => "/pin/vega-lite@v5.12.0-ujK64YZaLHcwzRN5lx1E/mode=imports,min/optimized/vega-lite.js",
            v5_13 => "/pin/vega-lite@v5.13.0-GkFo6HVxfKtvVL5RV8aE/mode=imports,min/optimized/vega-lite.js",
            v5_14 => "/pin/vega-lite@v5.14.1-0IRM1VigcIVzRzBRoLFR/mode=imports,min/optimized/vega-lite.js",
            v5_15 => "/pin/vega-lite@v5.15.1-lQeQs8sDPgFa9d7Jm3sd/mode=imports,min/optimized/vega-lite.js",
            v5_16 => "/pin/vega-lite@v5.16.1-q5OXwBBsYVZxkN0ArDu4/mode=imports,min/optimized/vega-lite.js"
        };
        path.to_string()
    }

    pub fn to_url(self) -> String {
        format!("{}{}", SKYPACK_URL, self.to_path())
    }

    pub fn to_semver(self) -> &'static str {
        use VlVersion::*;
        match self {
            v4_17 => "4.17",
            v5_8 => "5.8",
            v5_9 => "5.9",
            v5_10 => "5.10",
            v5_11 => "5.11",
            v5_12 => "5.12",
            v5_13 => "5.13",
            v5_14 => "5.14",
            v5_15 => "5.15",
            v5_16 => "5.16",
        }
    }
}

impl Default for VlVersion {
    fn default() -> Self {
        VlVersion::from_str("5.16").unwrap()
    }
}

impl FromStr for VlVersion {
    type Err = AnyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "4.17" | "v4.17" | "4_17" | "v4_17" => Self::v4_17,
            "5.8" | "v5.8" | "5_8" | "v5_8" => Self::v5_8,
            "5.9" | "v5.9" | "5_9" | "v5_9" => Self::v5_9,
            "5.10" | "v5.10" | "5_10" | "v5_10" => Self::v5_10,
            "5.11" | "v5.11" | "5_11" | "v5_11" => Self::v5_11,
            "5.12" | "v5.12" | "5_12" | "v5_12" => Self::v5_12,
            "5.13" | "v5.13" | "5_13" | "v5_13" => Self::v5_13,
            "5.14" | "v5.14" | "5_14" | "v5_14" => Self::v5_14,
            "5.15" | "v5.15" | "5_15" | "v5_15" => Self::v5_15,
            "5.16" | "v5.16" | "5_16" | "v5_16" => Self::v5_16,
            _ => bail!("Unsupported Vega-Lite version string {}", s),
        })
    }
}

pub const VL_VERSIONS: &[VlVersion] = &[
    VlVersion::v4_17,
    VlVersion::v5_8,
    VlVersion::v5_9,
    VlVersion::v5_10,
    VlVersion::v5_11,
    VlVersion::v5_12,
    VlVersion::v5_13,
    VlVersion::v5_14,
    VlVersion::v5_15,
    VlVersion::v5_16,
];

pub fn build_import_map() -> HashMap<String, String> {
    let mut m: HashMap<String, String> = HashMap::new();
    m.insert("/-/clone@v2.1.2-inH2VLNzDGiYU9HUWyZM/dist=es2020,mode=imports,min/optimized/clone.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/clone@v2.1.2-inH2VLNzDGiYU9HUWyZM/dist=es2020,mode=imports,min/optimized/clone.js").to_string());
    m.insert("/-/d3-array@v3.1.1-Ibshj34oOmCw8da1RLSW/dist=es2020,mode=imports,min/optimized/d3-array.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/d3-array@v3.1.1-Ibshj34oOmCw8da1RLSW/dist=es2020,mode=imports,min/optimized/d3-array.js").to_string());
    m.insert("/-/d3-array@v3.2.0-zhkQfnMQ2ct1k4iWdZiH/dist=es2020,mode=imports,min/optimized/d3-array.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/d3-array@v3.2.0-zhkQfnMQ2ct1k4iWdZiH/dist=es2020,mode=imports,min/optimized/d3-array.js").to_string());
    m.insert("/-/d3-array@v3.2.1-hGTrvmvXYXil9KElf3uD/dist=es2020,mode=imports,min/optimized/d3-array.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/d3-array@v3.2.1-hGTrvmvXYXil9KElf3uD/dist=es2020,mode=imports,min/optimized/d3-array.js").to_string());
    m.insert("/-/d3-array@v3.2.2-mREp9VYvdluM4XTkemzD/dist=es2020,mode=imports,min/optimized/d3-array.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/d3-array@v3.2.2-mREp9VYvdluM4XTkemzD/dist=es2020,mode=imports,min/optimized/d3-array.js").to_string());
    m.insert("/-/d3-array@v3.2.3-YYAnwYSQZgGML08QhecQ/dist=es2020,mode=imports,min/optimized/d3-array.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/d3-array@v3.2.3-YYAnwYSQZgGML08QhecQ/dist=es2020,mode=imports,min/optimized/d3-array.js").to_string());
    m.insert("/-/d3-color@v3.0.1-PMFX4FeSjgNPlkOOfuhz/dist=es2020,mode=imports,min/optimized/d3-color.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/d3-color@v3.0.1-PMFX4FeSjgNPlkOOfuhz/dist=es2020,mode=imports,min/optimized/d3-color.js").to_string());
    m.insert("/-/d3-color@v3.1.0-MWHDMwd2Pvp3NFjvrHgn/dist=es2020,mode=imports,min/optimized/d3-color.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/d3-color@v3.1.0-MWHDMwd2Pvp3NFjvrHgn/dist=es2020,mode=imports,min/optimized/d3-color.js").to_string());
    m.insert("/-/d3-delaunay@v6.0.2-W2uPNrmySJLg3U5jTBxc/dist=es2020,mode=imports,min/optimized/d3-delaunay.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/d3-delaunay@v6.0.2-W2uPNrmySJLg3U5jTBxc/dist=es2020,mode=imports,min/optimized/d3-delaunay.js").to_string());
    m.insert("/-/d3-dispatch@v3.0.1-v6nbfqO2iWOSwp77fYdB/dist=es2020,mode=imports,min/optimized/d3-dispatch.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/d3-dispatch@v3.0.1-v6nbfqO2iWOSwp77fYdB/dist=es2020,mode=imports,min/optimized/d3-dispatch.js").to_string());
    m.insert("/-/d3-dsv@v3.0.1-u1xCRjaLJc0qqv1Z5ERe/dist=es2020,mode=imports,min/optimized/d3-dsv.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/d3-dsv@v3.0.1-u1xCRjaLJc0qqv1Z5ERe/dist=es2020,mode=imports,min/optimized/d3-dsv.js").to_string());
    m.insert("/-/d3-force@v3.0.0-cshj62qMoyIGNIXoil9u/dist=es2020,mode=imports,min/optimized/d3-force.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/d3-force@v3.0.0-cshj62qMoyIGNIXoil9u/dist=es2020,mode=imports,min/optimized/d3-force.js").to_string());
    m.insert("/-/d3-format@v3.0.1-LyjmeNp0E5YokPvOjBaD/dist=es2020,mode=imports,min/optimized/d3-format.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/d3-format@v3.0.1-LyjmeNp0E5YokPvOjBaD/dist=es2020,mode=imports,min/optimized/d3-format.js").to_string());
    m.insert("/-/d3-format@v3.1.0-D5wAD2odDPNNWsKloKgL/dist=es2020,mode=imports,min/optimized/d3-format.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/d3-format@v3.1.0-D5wAD2odDPNNWsKloKgL/dist=es2020,mode=imports,min/optimized/d3-format.js").to_string());
    m.insert("/-/d3-geo-projection@v4.0.0-5Hhxj2zKHEqWYAQIFo3r/dist=es2020,mode=imports,min/optimized/d3-geo-projection.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/d3-geo-projection@v4.0.0-5Hhxj2zKHEqWYAQIFo3r/dist=es2020,mode=imports,min/optimized/d3-geo-projection.js").to_string());
    m.insert("/-/d3-geo@v3.0.1-kwyelOm8gApBxT2oVVB9/dist=es2020,mode=imports,min/optimized/d3-geo.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/d3-geo@v3.0.1-kwyelOm8gApBxT2oVVB9/dist=es2020,mode=imports,min/optimized/d3-geo.js").to_string());
    m.insert("/-/d3-geo@v3.1.0-6gCuCN3p6hXOeZDWcbjw/dist=es2020,mode=imports,min/optimized/d3-geo.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/d3-geo@v3.1.0-6gCuCN3p6hXOeZDWcbjw/dist=es2020,mode=imports,min/optimized/d3-geo.js").to_string());
    m.insert("/-/d3-hierarchy@v3.1.2-wx7sW10pU4OkfBLgCDCU/dist=es2020,mode=imports,min/optimized/d3-hierarchy.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/d3-hierarchy@v3.1.2-wx7sW10pU4OkfBLgCDCU/dist=es2020,mode=imports,min/optimized/d3-hierarchy.js").to_string());
    m.insert("/-/d3-interpolate@v3.0.1-i9AsUdFHwyaukRBWNe8d/dist=es2020,mode=imports,min/optimized/d3-interpolate.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/d3-interpolate@v3.0.1-i9AsUdFHwyaukRBWNe8d/dist=es2020,mode=imports,min/optimized/d3-interpolate.js").to_string());
    m.insert("/-/d3-path@v3.1.0-nHaUoYzlRDYONpece9h0/dist=es2020,mode=imports,min/optimized/d3-path.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/d3-path@v3.1.0-nHaUoYzlRDYONpece9h0/dist=es2020,mode=imports,min/optimized/d3-path.js").to_string());
    m.insert("/-/d3-quadtree@v3.0.1-sMtwlDFghZGCTQ3UxKMT/dist=es2020,mode=imports,min/optimized/d3-quadtree.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/d3-quadtree@v3.0.1-sMtwlDFghZGCTQ3UxKMT/dist=es2020,mode=imports,min/optimized/d3-quadtree.js").to_string());
    m.insert("/-/d3-scale@v4.0.2-qUv67mnQQKwRMEsPRKcO/dist=es2020,mode=imports,min/optimized/d3-scale.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/d3-scale@v4.0.2-qUv67mnQQKwRMEsPRKcO/dist=es2020,mode=imports,min/optimized/d3-scale.js").to_string());
    m.insert("/-/d3-shape@v3.2.0-jvLE9CjF3Vp4eEpVme8s/dist=es2020,mode=imports,min/optimized/d3-shape.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/d3-shape@v3.2.0-jvLE9CjF3Vp4eEpVme8s/dist=es2020,mode=imports,min/optimized/d3-shape.js").to_string());
    m.insert("/-/d3-time-format@v4.0.0-A7vYeSqgWxeXXSpz1rEp/dist=es2020,mode=imports,min/optimized/d3-time-format.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/d3-time-format@v4.0.0-A7vYeSqgWxeXXSpz1rEp/dist=es2020,mode=imports,min/optimized/d3-time-format.js").to_string());
    m.insert("/-/d3-time-format@v4.1.0-f8eZV7eLtGIxvK8uvO3o/dist=es2020,mode=imports,min/optimized/d3-time-format.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/d3-time-format@v4.1.0-f8eZV7eLtGIxvK8uvO3o/dist=es2020,mode=imports,min/optimized/d3-time-format.js").to_string());
    m.insert("/-/d3-time@v3.0.0-Ww07wkuPsE2c8Ac33BKQ/dist=es2020,mode=imports,min/optimized/d3-time.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/d3-time@v3.0.0-Ww07wkuPsE2c8Ac33BKQ/dist=es2020,mode=imports,min/optimized/d3-time.js").to_string());
    m.insert("/-/d3-time@v3.1.0-hkusO1LcNQpH1ccXwop7/dist=es2020,mode=imports,min/optimized/d3-time.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/d3-time@v3.1.0-hkusO1LcNQpH1ccXwop7/dist=es2020,mode=imports,min/optimized/d3-time.js").to_string());
    m.insert("/-/d3-timer@v3.0.1-O0QpYiI2jhOLEJodLnN1/dist=es2020,mode=imports,min/optimized/d3-timer.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/d3-timer@v3.0.1-O0QpYiI2jhOLEJodLnN1/dist=es2020,mode=imports,min/optimized/d3-timer.js").to_string());
    m.insert("/-/delaunator@v5.0.0-60DR2BxrKUGp07uAwKTO/dist=es2020,mode=imports,min/optimized/delaunator.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/delaunator@v5.0.0-60DR2BxrKUGp07uAwKTO/dist=es2020,mode=imports,min/optimized/delaunator.js").to_string());
    m.insert("/-/fast-deep-equal@v3.1.3-ysejKs1WDEDPxUJhgGoP/dist=es2020,mode=imports,min/optimized/fast-deep-equal.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/fast-deep-equal@v3.1.3-ysejKs1WDEDPxUJhgGoP/dist=es2020,mode=imports,min/optimized/fast-deep-equal.js").to_string());
    m.insert("/-/fast-json-patch@v3.1.1-IjacxII42OC4A6OXhkDe/dist=es2020,mode=imports,min/optimized/fast-json-patch.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/fast-json-patch@v3.1.1-IjacxII42OC4A6OXhkDe/dist=es2020,mode=imports,min/optimized/fast-json-patch.js").to_string());
    m.insert("/-/fast-json-stable-stringify@v2.1.0-HLgsuOtxPikt0pw16nth/dist=es2020,mode=imports,min/optimized/fast-json-stable-stringify.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/fast-json-stable-stringify@v2.1.0-HLgsuOtxPikt0pw16nth/dist=es2020,mode=imports,min/optimized/fast-json-stable-stringify.js").to_string());
    m.insert("/-/internmap@v2.0.3-GWZlRrRMFcDlELwTQEZq/dist=es2020,mode=imports,min/optimized/internmap.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/internmap@v2.0.3-GWZlRrRMFcDlELwTQEZq/dist=es2020,mode=imports,min/optimized/internmap.js").to_string());
    m.insert("/-/json-stringify-pretty-compact@v3.0.0-RM0i5NMwoiFhg7YNuXef/dist=es2020,mode=imports,min/optimized/json-stringify-pretty-compact.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/json-stringify-pretty-compact@v3.0.0-RM0i5NMwoiFhg7YNuXef/dist=es2020,mode=imports,min/optimized/json-stringify-pretty-compact.js").to_string());
    m.insert("/-/lodash.debounce@v4.0.8-aOLIwnE2RethWPrEzTeR/dist=es2020,mode=imports,min/optimized/lodash.debounce.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/lodash.debounce@v4.0.8-aOLIwnE2RethWPrEzTeR/dist=es2020,mode=imports,min/optimized/lodash.debounce.js").to_string());
    m.insert("/-/robust-predicates@v3.0.1-HLqPGJ72Lz4vKeF3T1y8/dist=es2020,mode=imports,min/optimized/robust-predicates.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/robust-predicates@v3.0.1-HLqPGJ72Lz4vKeF3T1y8/dist=es2020,mode=imports,min/optimized/robust-predicates.js").to_string());
    m.insert("/-/topojson-client@v3.1.0-fyhI24JwGwsqazuuSEoq/dist=es2020,mode=imports,min/optimized/topojson-client.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/topojson-client@v3.1.0-fyhI24JwGwsqazuuSEoq/dist=es2020,mode=imports,min/optimized/topojson-client.js").to_string());
    m.insert("/-/vega-canvas@v1.2.7-hCEcvULuKIOqBVGX1Tn8/dist=es2020,mode=imports,min/optimized/vega-canvas.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-canvas@v1.2.7-hCEcvULuKIOqBVGX1Tn8/dist=es2020,mode=imports,min/optimized/vega-canvas.js").to_string());
    m.insert("/-/vega-crossfilter@v4.1.1-0AUAt51ACgvwsfDQr3R4/dist=es2020,mode=imports,min/optimized/vega-crossfilter.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-crossfilter@v4.1.1-0AUAt51ACgvwsfDQr3R4/dist=es2020,mode=imports,min/optimized/vega-crossfilter.js").to_string());
    m.insert("/-/vega-dataflow@v5.7.5-asKYS4gpPLMPf64pSozt/dist=es2020,mode=imports,min/optimized/vega-dataflow.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-dataflow@v5.7.5-asKYS4gpPLMPf64pSozt/dist=es2020,mode=imports,min/optimized/vega-dataflow.js").to_string());
    m.insert("/-/vega-embed@v6.23.0-Fpmq39rehEH8HWtd6nzv/dist=es2020,mode=imports,min/optimized/vega-embed.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-embed@v6.23.0-Fpmq39rehEH8HWtd6nzv/dist=es2020,mode=imports,min/optimized/vega-embed.js").to_string());
    m.insert("/-/vega-encode@v4.9.2-73LzBc3ioSKmAYWtrrRd/dist=es2020,mode=imports,min/optimized/vega-encode.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-encode@v4.9.2-73LzBc3ioSKmAYWtrrRd/dist=es2020,mode=imports,min/optimized/vega-encode.js").to_string());
    m.insert("/-/vega-event-selector@v2.0.6-fTsDie9ajW0bqk8q646l/dist=es2020,mode=imports,min/optimized/vega-event-selector.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-event-selector@v2.0.6-fTsDie9ajW0bqk8q646l/dist=es2020,mode=imports,min/optimized/vega-event-selector.js").to_string());
    m.insert("/-/vega-event-selector@v3.0.1-UgiEAWJA4WQL4DTKnV4R/dist=es2020,mode=imports,min/optimized/vega-event-selector.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-event-selector@v3.0.1-UgiEAWJA4WQL4DTKnV4R/dist=es2020,mode=imports,min/optimized/vega-event-selector.js").to_string());
    m.insert("/-/vega-expression@v3.0.1-A7mhk5wjrSNOpQiSL1LP/dist=es2020,mode=imports,min/optimized/vega-expression.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-expression@v3.0.1-A7mhk5wjrSNOpQiSL1LP/dist=es2020,mode=imports,min/optimized/vega-expression.js").to_string());
    m.insert("/-/vega-expression@v5.0.1-M3HJnXNS0AVPna9ohQFr/dist=es2020,mode=imports,min/optimized/vega-expression.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-expression@v5.0.1-M3HJnXNS0AVPna9ohQFr/dist=es2020,mode=imports,min/optimized/vega-expression.js").to_string());
    m.insert("/-/vega-expression@v5.1.0-2VnsdaYPtQ6Pi5w8JxN8/dist=es2020,mode=imports,min/optimized/vega-expression.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-expression@v5.1.0-2VnsdaYPtQ6Pi5w8JxN8/dist=es2020,mode=imports,min/optimized/vega-expression.js").to_string());
    m.insert("/-/vega-force@v4.2.0-CTEP5XRxhUGWUZM8MwQs/dist=es2020,mode=imports,min/optimized/vega-force.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-force@v4.2.0-CTEP5XRxhUGWUZM8MwQs/dist=es2020,mode=imports,min/optimized/vega-force.js").to_string());
    m.insert("/-/vega-format@v1.1.1-MIkBcxVtnuOzCt2MeOxi/dist=es2020,mode=imports,min/optimized/vega-format.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-format@v1.1.1-MIkBcxVtnuOzCt2MeOxi/dist=es2020,mode=imports,min/optimized/vega-format.js").to_string());
    m.insert("/-/vega-functions@v5.13.1-5gFjwySEK8datc3yFaKG/dist=es2020,mode=imports,min/optimized/vega-functions.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-functions@v5.13.1-5gFjwySEK8datc3yFaKG/dist=es2020,mode=imports,min/optimized/vega-functions.js").to_string());
    m.insert("/-/vega-functions@v5.13.2-D3m7Mll472SfG8JZdJut/dist=es2020,mode=imports,min/optimized/vega-functions.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-functions@v5.13.2-D3m7Mll472SfG8JZdJut/dist=es2020,mode=imports,min/optimized/vega-functions.js").to_string());
    m.insert("/-/vega-geo@v4.4.1-8TUdHGrJmGMaKM9x1bwB/dist=es2020,mode=imports,min/optimized/vega-geo.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-geo@v4.4.1-8TUdHGrJmGMaKM9x1bwB/dist=es2020,mode=imports,min/optimized/vega-geo.js").to_string());
    m.insert("/-/vega-hierarchy@v4.1.1-UOm1ATDeVOQ6Lqji63hi/dist=es2020,mode=imports,min/optimized/vega-hierarchy.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-hierarchy@v4.1.1-UOm1ATDeVOQ6Lqji63hi/dist=es2020,mode=imports,min/optimized/vega-hierarchy.js").to_string());
    m.insert("/-/vega-interpreter@v1.0.5-xGayK8haM1EVgaoW7oOi/dist=es2020,mode=imports,min/optimized/vega-interpreter.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-interpreter@v1.0.5-xGayK8haM1EVgaoW7oOi/dist=es2020,mode=imports,min/optimized/vega-interpreter.js").to_string());
    m.insert("/-/vega-label@v1.2.1-hEHqe293fM7lKP2OHb0o/dist=es2020,mode=imports,min/optimized/vega-label.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-label@v1.2.1-hEHqe293fM7lKP2OHb0o/dist=es2020,mode=imports,min/optimized/vega-label.js").to_string());
    m.insert("/-/vega-lite@v4.17.0-ycT3UrEO81NWOPVKlbjt/dist=es2020,mode=imports,min/optimized/vega-lite.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-lite@v4.17.0-ycT3UrEO81NWOPVKlbjt/dist=es2020,mode=imports,min/optimized/vega-lite.js").to_string());
    m.insert("/-/vega-lite@v5.10.0-Vm0dgr6cpOyUiTjlPzt9/dist=es2020,mode=imports,min/optimized/vega-lite.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-lite@v5.10.0-Vm0dgr6cpOyUiTjlPzt9/dist=es2020,mode=imports,min/optimized/vega-lite.js").to_string());
    m.insert("/-/vega-lite@v5.11.1-Q5Jhmb2acmWm03IObXvn/dist=es2020,mode=imports,min/optimized/vega-lite.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-lite@v5.11.1-Q5Jhmb2acmWm03IObXvn/dist=es2020,mode=imports,min/optimized/vega-lite.js").to_string());
    m.insert("/-/vega-lite@v5.12.0-ujK64YZaLHcwzRN5lx1E/dist=es2020,mode=imports,min/optimized/vega-lite.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-lite@v5.12.0-ujK64YZaLHcwzRN5lx1E/dist=es2020,mode=imports,min/optimized/vega-lite.js").to_string());
    m.insert("/-/vega-lite@v5.13.0-GkFo6HVxfKtvVL5RV8aE/dist=es2020,mode=imports,min/optimized/vega-lite.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-lite@v5.13.0-GkFo6HVxfKtvVL5RV8aE/dist=es2020,mode=imports,min/optimized/vega-lite.js").to_string());
    m.insert("/-/vega-lite@v5.14.1-0IRM1VigcIVzRzBRoLFR/dist=es2020,mode=imports,min/optimized/vega-lite.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-lite@v5.14.1-0IRM1VigcIVzRzBRoLFR/dist=es2020,mode=imports,min/optimized/vega-lite.js").to_string());
    m.insert("/-/vega-lite@v5.15.0-l27jdbpFztiLqmj2Waun/dist=es2020,mode=imports,min/optimized/vega-lite.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-lite@v5.15.0-l27jdbpFztiLqmj2Waun/dist=es2020,mode=imports,min/optimized/vega-lite.js").to_string());
    m.insert("/-/vega-lite@v5.15.1-lQeQs8sDPgFa9d7Jm3sd/dist=es2020,mode=imports,min/optimized/vega-lite.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-lite@v5.15.1-lQeQs8sDPgFa9d7Jm3sd/dist=es2020,mode=imports,min/optimized/vega-lite.js").to_string());
    m.insert("/-/vega-lite@v5.16.1-q5OXwBBsYVZxkN0ArDu4/dist=es2020,mode=imports,min/optimized/vega-lite.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-lite@v5.16.1-q5OXwBBsYVZxkN0ArDu4/dist=es2020,mode=imports,min/optimized/vega-lite.js").to_string());
    m.insert("/-/vega-lite@v5.8.0-4snbURNltT4se5LjMOKF/dist=es2020,mode=imports,min/optimized/vega-lite.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-lite@v5.8.0-4snbURNltT4se5LjMOKF/dist=es2020,mode=imports,min/optimized/vega-lite.js").to_string());
    m.insert("/-/vega-lite@v5.9.3-QyXScylQe0TTmb9DRCES/dist=es2020,mode=imports,min/optimized/vega-lite.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-lite@v5.9.3-QyXScylQe0TTmb9DRCES/dist=es2020,mode=imports,min/optimized/vega-lite.js").to_string());
    m.insert("/-/vega-loader@v4.5.1-e2JpneCYErTzObWVOVxs/dist=es2020,mode=imports,min/optimized/vega-loader.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-loader@v4.5.1-e2JpneCYErTzObWVOVxs/dist=es2020,mode=imports,min/optimized/vega-loader.js").to_string());
    m.insert("/-/vega-parser@v6.2.0-Ld7hvHcZsTPXKLjR4bzT/dist=es2020,mode=imports,min/optimized/vega-parser.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-parser@v6.2.0-Ld7hvHcZsTPXKLjR4bzT/dist=es2020,mode=imports,min/optimized/vega-parser.js").to_string());
    m.insert("/-/vega-projection@v1.6.0-lGjH1T1qHepW8BpZZrrV/dist=es2020,mode=imports,min/optimized/vega-projection.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-projection@v1.6.0-lGjH1T1qHepW8BpZZrrV/dist=es2020,mode=imports,min/optimized/vega-projection.js").to_string());
    m.insert("/-/vega-regression@v1.2.0-qmJkAzTyLtlu9jZBRY67/dist=es2020,mode=imports,min/optimized/vega-regression.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-regression@v1.2.0-qmJkAzTyLtlu9jZBRY67/dist=es2020,mode=imports,min/optimized/vega-regression.js").to_string());
    m.insert("/-/vega-runtime@v6.1.4-aFKCWR0DKdVqzAheQZ7x/dist=es2020,mode=imports,min/optimized/vega-runtime.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-runtime@v6.1.4-aFKCWR0DKdVqzAheQZ7x/dist=es2020,mode=imports,min/optimized/vega-runtime.js").to_string());
    m.insert("/-/vega-scale@v7.3.0-RE8rHwByiw8oUoAe4pNs/dist=es2020,mode=imports,min/optimized/vega-scale.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-scale@v7.3.0-RE8rHwByiw8oUoAe4pNs/dist=es2020,mode=imports,min/optimized/vega-scale.js").to_string());
    m.insert("/-/vega-scenegraph@v4.10.2-W1dltMWN7mO3TARazJA1/dist=es2020,mode=imports,min/optimized/vega-scenegraph.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-scenegraph@v4.10.2-W1dltMWN7mO3TARazJA1/dist=es2020,mode=imports,min/optimized/vega-scenegraph.js").to_string());
    m.insert("/-/vega-schema-url-parser@v2.2.0-YmXJGRcKOXOac3VG4xfw/dist=es2020,mode=imports,min/optimized/vega-schema-url-parser.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-schema-url-parser@v2.2.0-YmXJGRcKOXOac3VG4xfw/dist=es2020,mode=imports,min/optimized/vega-schema-url-parser.js").to_string());
    m.insert("/-/vega-selections@v5.4.1-4yIDtA9NY6vzeN00RWao/dist=es2020,mode=imports,min/optimized/vega-selections.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-selections@v5.4.1-4yIDtA9NY6vzeN00RWao/dist=es2020,mode=imports,min/optimized/vega-selections.js").to_string());
    m.insert("/-/vega-statistics@v1.8.1-mXKADMfVQufwILOgkTiI/dist=es2020,mode=imports,min/optimized/vega-statistics.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-statistics@v1.8.1-mXKADMfVQufwILOgkTiI/dist=es2020,mode=imports,min/optimized/vega-statistics.js").to_string());
    m.insert("/-/vega-statistics@v1.9.0-Qw8CjSQVQOg2M6VMgsme/dist=es2020,mode=imports,min/optimized/vega-statistics.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-statistics@v1.9.0-Qw8CjSQVQOg2M6VMgsme/dist=es2020,mode=imports,min/optimized/vega-statistics.js").to_string());
    m.insert("/-/vega-themes@v2.14.0-RvUmNETlVH2y3yQM1y36/dist=es2020,mode=imports,min/optimized/vega-themes.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-themes@v2.14.0-RvUmNETlVH2y3yQM1y36/dist=es2020,mode=imports,min/optimized/vega-themes.js").to_string());
    m.insert("/-/vega-time@v2.1.1-Q1TxQbneCNdh5ryZm2Gf/dist=es2020,mode=imports,min/optimized/vega-time.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-time@v2.1.1-Q1TxQbneCNdh5ryZm2Gf/dist=es2020,mode=imports,min/optimized/vega-time.js").to_string());
    m.insert("/-/vega-tooltip@v0.33.0-DfMhYyd4NOGdbNfmDNiw/dist=es2020,mode=imports,min/optimized/vega-tooltip.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-tooltip@v0.33.0-DfMhYyd4NOGdbNfmDNiw/dist=es2020,mode=imports,min/optimized/vega-tooltip.js").to_string());
    m.insert("/-/vega-transforms@v4.10.2-ZwuxbjLubhdUwIKHKtlL/dist=es2020,mode=imports,min/optimized/vega-transforms.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-transforms@v4.10.2-ZwuxbjLubhdUwIKHKtlL/dist=es2020,mode=imports,min/optimized/vega-transforms.js").to_string());
    m.insert("/-/vega-util@v1.16.1-QtdV0YLGVmjtkzFiPbzm/dist=es2020,mode=imports,min/optimized/vega-util.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-util@v1.16.1-QtdV0YLGVmjtkzFiPbzm/dist=es2020,mode=imports,min/optimized/vega-util.js").to_string());
    m.insert("/-/vega-util@v1.17.0-uRskU0IBL2vWCP4Va8OC/dist=es2020,mode=imports,min/optimized/vega-util.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-util@v1.17.0-uRskU0IBL2vWCP4Va8OC/dist=es2020,mode=imports,min/optimized/vega-util.js").to_string());
    m.insert("/-/vega-util@v1.17.1-uwuqwLZrXXBeO0DFYRgh/dist=es2020,mode=imports,min/optimized/vega-util.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-util@v1.17.1-uwuqwLZrXXBeO0DFYRgh/dist=es2020,mode=imports,min/optimized/vega-util.js").to_string());
    m.insert("/-/vega-util@v1.17.2-LUfkDhormMyfWqy3Ts6U/dist=es2020,mode=imports,min/optimized/vega-util.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-util@v1.17.2-LUfkDhormMyfWqy3Ts6U/dist=es2020,mode=imports,min/optimized/vega-util.js").to_string());
    m.insert("/-/vega-view-transforms@v4.5.9-LiB26zIbxiHZW70fnrDI/dist=es2020,mode=imports,min/optimized/vega-view-transforms.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-view-transforms@v4.5.9-LiB26zIbxiHZW70fnrDI/dist=es2020,mode=imports,min/optimized/vega-view-transforms.js").to_string());
    m.insert("/-/vega-view@v5.11.1-FQ9r1BvJOMHegkomXDyj/dist=es2020,mode=imports,min/optimized/vega-view.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-view@v5.11.1-FQ9r1BvJOMHegkomXDyj/dist=es2020,mode=imports,min/optimized/vega-view.js").to_string());
    m.insert("/-/vega-voronoi@v4.2.1-NrRTxuT3DOJ52CnCL8nc/dist=es2020,mode=imports,min/optimized/vega-voronoi.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-voronoi@v4.2.1-NrRTxuT3DOJ52CnCL8nc/dist=es2020,mode=imports,min/optimized/vega-voronoi.js").to_string());
    m.insert("/-/vega-wordcloud@v4.1.4-J5ewLsaJzX7o65uuoGqD/dist=es2020,mode=imports,min/optimized/vega-wordcloud.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega-wordcloud@v4.1.4-J5ewLsaJzX7o65uuoGqD/dist=es2020,mode=imports,min/optimized/vega-wordcloud.js").to_string());
    m.insert("/-/vega@v5.25.0-r16knbfAAfBFDoUvoc7K/dist=es2020,mode=imports,min/optimized/vega.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/-/vega@v5.25.0-r16knbfAAfBFDoUvoc7K/dist=es2020,mode=imports,min/optimized/vega.js").to_string());
    m.insert("/pin/lodash.debounce@v4.0.8-aOLIwnE2RethWPrEzTeR/mode=imports,min/optimized/lodash.debounce.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/pin/lodash.debounce@v4.0.8-aOLIwnE2RethWPrEzTeR/mode=imports,min/optimized/lodash.debounce.js").to_string());
    m.insert("/pin/vega-embed@v6.23.0-Fpmq39rehEH8HWtd6nzv/mode=imports,min/optimized/vega-embed.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/pin/vega-embed@v6.23.0-Fpmq39rehEH8HWtd6nzv/mode=imports,min/optimized/vega-embed.js").to_string());
    m.insert("/pin/vega-lite@v4.17.0-ycT3UrEO81NWOPVKlbjt/mode=imports,min/optimized/vega-lite.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/pin/vega-lite@v4.17.0-ycT3UrEO81NWOPVKlbjt/mode=imports,min/optimized/vega-lite.js").to_string());
    m.insert("/pin/vega-lite@v5.10.0-Vm0dgr6cpOyUiTjlPzt9/mode=imports,min/optimized/vega-lite.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/pin/vega-lite@v5.10.0-Vm0dgr6cpOyUiTjlPzt9/mode=imports,min/optimized/vega-lite.js").to_string());
    m.insert("/pin/vega-lite@v5.11.1-Q5Jhmb2acmWm03IObXvn/mode=imports,min/optimized/vega-lite.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/pin/vega-lite@v5.11.1-Q5Jhmb2acmWm03IObXvn/mode=imports,min/optimized/vega-lite.js").to_string());
    m.insert("/pin/vega-lite@v5.12.0-ujK64YZaLHcwzRN5lx1E/mode=imports,min/optimized/vega-lite.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/pin/vega-lite@v5.12.0-ujK64YZaLHcwzRN5lx1E/mode=imports,min/optimized/vega-lite.js").to_string());
    m.insert("/pin/vega-lite@v5.13.0-GkFo6HVxfKtvVL5RV8aE/mode=imports,min/optimized/vega-lite.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/pin/vega-lite@v5.13.0-GkFo6HVxfKtvVL5RV8aE/mode=imports,min/optimized/vega-lite.js").to_string());
    m.insert("/pin/vega-lite@v5.14.1-0IRM1VigcIVzRzBRoLFR/mode=imports,min/optimized/vega-lite.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/pin/vega-lite@v5.14.1-0IRM1VigcIVzRzBRoLFR/mode=imports,min/optimized/vega-lite.js").to_string());
    m.insert("/pin/vega-lite@v5.15.1-lQeQs8sDPgFa9d7Jm3sd/mode=imports,min/optimized/vega-lite.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/pin/vega-lite@v5.15.1-lQeQs8sDPgFa9d7Jm3sd/mode=imports,min/optimized/vega-lite.js").to_string());
    m.insert("/pin/vega-lite@v5.16.1-q5OXwBBsYVZxkN0ArDu4/mode=imports,min/optimized/vega-lite.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/pin/vega-lite@v5.16.1-q5OXwBBsYVZxkN0ArDu4/mode=imports,min/optimized/vega-lite.js").to_string());
    m.insert("/pin/vega-lite@v5.8.0-4snbURNltT4se5LjMOKF/mode=imports,min/optimized/vega-lite.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/pin/vega-lite@v5.8.0-4snbURNltT4se5LjMOKF/mode=imports,min/optimized/vega-lite.js").to_string());
    m.insert("/pin/vega-lite@v5.9.3-QyXScylQe0TTmb9DRCES/mode=imports,min/optimized/vega-lite.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/pin/vega-lite@v5.9.3-QyXScylQe0TTmb9DRCES/mode=imports,min/optimized/vega-lite.js").to_string());
    m.insert("/pin/vega-themes@v2.14.0-RvUmNETlVH2y3yQM1y36/mode=imports,min/optimized/vega-themes.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/pin/vega-themes@v2.14.0-RvUmNETlVH2y3yQM1y36/mode=imports,min/optimized/vega-themes.js").to_string());
    m.insert("/pin/vega@v5.25.0-r16knbfAAfBFDoUvoc7K/mode=imports,min/optimized/vega.js".to_string(), include_str!("../../vendor/cdn.skypack.dev/pin/vega@v5.25.0-r16knbfAAfBFDoUvoc7K/mode=imports,min/optimized/vega.js").to_string());
    m
}
