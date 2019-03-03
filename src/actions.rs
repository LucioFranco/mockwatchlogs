use crate::streams::{Group, Stream};
use crate::types::*;
use crate::{Body, Context, Response};
use serde_json::json;

pub fn describe_streams(
    context: &mut Context,
    request: DescribeLogStreamsRequest,
) -> Result<Response, ServiceError> {
    let group = context.groups.get(&request.log_group_name);

    let streams = if let Some(group) = group {
        group.streams.clone()
    } else {
        return Err(ServiceError::NotFound("Group not found".into()));
    };

    let streams = if let Some(prefix) = request.log_stream_name_prefix {
        streams
            .into_iter()
            .filter(|stream| stream.name.starts_with(prefix.as_str()))
            .map(|e| e.into())
            .collect()
    } else {
        streams.into_iter().map(|e| e.into()).collect()
    };

    let res = DescribeLogStreamsResponse {
        log_streams: Some(streams),
        next_token: None,
    };

    let body = serde_json::to_vec(&res).unwrap();
    Ok(Response::new(Body::from(body)))
}

pub fn create_group(
    context: &mut Context,
    request: CreateLogGroupRequest,
) -> Result<Response, ServiceError> {
    if let None = context.groups.get(&request.log_group_name) {
        context
            .groups
            .insert(request.log_group_name, Group::default());
        Ok(Response::new(Body::empty()))
    } else {
        Err(ServiceError::ResourceAlreadyExistsException)
    }
}

pub fn create_stream(
    context: &mut Context,
    request: CreateLogStreamRequest,
) -> Result<Response, ServiceError> {
    if let Some(ref mut group) = context.groups.get_mut(&request.log_group_name) {
        if let None = group
            .streams
            .iter()
            .find(|e| e.name == request.log_stream_name)
        {
            group.streams.push(Stream {
                name: request.log_stream_name.clone(),
                logs: Vec::new(),
            });
            Ok(Response::new(Body::empty()))
        } else {
            Err(ServiceError::ResourceAlreadyExistsException)
        }
    } else {
        Err(ServiceError::NotFound("Group not found".into()))
    }
}

pub fn put_logs(
    context: &mut Context,
    request: PutLogEventsRequest,
) -> Result<Response, ServiceError> {
    if let Some(ref mut group) = context.groups.get_mut(&request.log_group_name) {
        if let Some(stream) = group
            .streams
            .iter_mut()
            .find(|e| e.name == request.log_stream_name)
        {
            // TODO: validate logs?
            stream.logs.extend(request.log_events.clone());

            let res = PutLogEventsResponse {
                ..Default::default()
            };

            let body = serde_json::to_vec(&res).unwrap();
            Ok(Response::new(Body::from(body)))
        } else {
            Err(ServiceError::NotFound("Stream not found".into()))
        }
    } else {
        Err(ServiceError::NotFound("Group not found".into()))
    }
}

pub fn get_logs(
    context: &mut Context,
    request: GetLogEventsRequest,
) -> Result<Response, ServiceError> {
    if let Some(ref mut group) = context.groups.get_mut(&request.log_group_name) {
        if let Some(stream) = group
            .streams
            .iter()
            .find(|e| e.name == request.log_stream_name)
        {
            let logs = stream
                .logs
                .iter()
                .filter(|log| {
                    if let Some(start_time) = request.start_time {
                        log.timestamp >= start_time
                    } else {
                        true
                    }
                })
                .map(|l| OutputLogEvent {
                    message: Some(l.message.clone()),
                    timestamp: Some(l.timestamp),
                    ..Default::default()
                })
                .collect();

            let res = GetLogEventsResponse {
                events: Some(logs),
                ..Default::default()
            };

            let body = serde_json::to_vec(&res).unwrap();
            Ok(Response::new(Body::from(body)))
        } else {
            Err(ServiceError::NotFound("Stream not found".into()))
        }
    } else {
        Err(ServiceError::NotFound("Group not found".into()))
    }
}

pub enum ServiceError {
    NotFound(String),
    ResourceAlreadyExistsException,
}

impl From<ServiceError> for hyper::Response<hyper::Body> {
    fn from(e: ServiceError) -> Self {
        match e {
            ServiceError::NotFound(message) => {
                let json = json!({
                    "__type": "ResourceNotFoundException",
                    "message": message
                });

                let body = serde_json::to_vec(&json).unwrap();

                hyper::Response::builder()
                    .status(404)
                    .body(hyper::Body::from(body))
                    .unwrap()
            }
            ServiceError::ResourceAlreadyExistsException => {
                let json = json!({
                    "__type": "ResourceAlreadyExistsException",
                    "message": "Resource not found"
                });

                let body = serde_json::to_vec(&json).unwrap();

                hyper::Response::builder()
                    .status(404)
                    .body(hyper::Body::from(body))
                    .unwrap()
            }
        }
    }
}
