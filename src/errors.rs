use aws_sdk_dynamodb::types::SdkError;
use lambda_http;
// use aws_sdk_lambda;
use aws_sdk_dynamodb;
use serde_dynamo;
use hyper;

#[derive(Debug, Copy, Clone)]
pub enum CustomErrorType {
    AwsError,
    // LambdaError,
    HttpError,
    SdkError,
    SerdeError,
    HyperError,
}

#[derive(Debug)]
pub struct CustomError {
    pub message: Option<String>,
    pub err_type: CustomErrorType
}

impl CustomError {
    pub fn message(&self) -> String {
        match &self.message {
            Some(m) => m.clone(),
            None => String::from("")
        }
    }

    pub fn error_type(&self) -> CustomErrorType {
        self.err_type.clone()
    }
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<serde_dynamo::Error> for CustomError {
    fn from(err: serde_dynamo::Error) -> CustomError {
        CustomError {
            message: Some(err.to_string()),
            err_type: CustomErrorType::AwsError
        }
    }
}

impl From<aws_sdk_dynamodb::Error> for CustomError {
    fn from(err: aws_sdk_dynamodb::Error) -> CustomError {
        CustomError {
            message: Some(err.to_string()),
            err_type: CustomErrorType::AwsError
        }
    }
}

impl From<hyper::Error> for CustomError {
    fn from(err: hyper::Error) -> CustomError {
        CustomError {
            message: Some(err.to_string()),
            err_type: CustomErrorType::HyperError
        }
    }
}

// impl From<aws_sdk_lambda::Error> for CustomError {
//     fn from(err: aws_sdk_lambda::Error) -> CustomError {
//         CustomError {
//             message: Some(err.to_string()),
//             err_type: CustomErrorType::LambdaError
//         }
//     }
// }

impl From<lambda_http::Error> for CustomError {
    fn from(err: lambda_http::Error) -> CustomError {
        CustomError {
            message: Some(err.to_string()),
            err_type: CustomErrorType::HttpError
        }
    }
}

impl From<lambda_http::http::Error> for CustomError {
    fn from(err: lambda_http::http::Error) -> CustomError {
        CustomError {
            message: Some(err.to_string()),
            err_type: CustomErrorType::HttpError
        }
    }
}

impl<E: std::fmt::Debug> From<SdkError<E>> for CustomError {
    fn from(err: SdkError<E>) -> CustomError {
        CustomError {
            message: Some(format!("{:?}", err)),
            err_type: CustomErrorType::SdkError
        }
    }
}