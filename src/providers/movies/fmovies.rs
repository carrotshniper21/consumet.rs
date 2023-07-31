use crate::extractors::{StreamTape, VizCloud};
use crate::models::{
    IEpisodeServer, IMovieEpisode, IMovieInfo, IMovieResult, ISearch, ISource, MovieParser,
    ProxyConfig, StreamingServers, TvType, BaseParser
};
use crate::utils::util_funcs::UtilFuncs;

use serde::Deserialize;

pub struct Fmovies;