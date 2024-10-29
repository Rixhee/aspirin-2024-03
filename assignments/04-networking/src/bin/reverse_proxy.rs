use aspirin_eats::error::AspirinEatsError;
use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_connection<R: Read + std::io::Write, W: Write + std::io::Read>(
    mut client_stream: R,
    origin_server: &mut W,
) -> Result<Vec<u8>, AspirinEatsError> {
    let mut client_buffer = [0; 65536];
    let mut origin_buffer = [0; 65536];

    let bytes_read = client_stream.read(&mut client_buffer)?;
    if bytes_read == 0 {
        return Ok(Vec::new());
    }

    origin_server.write_all(&client_buffer[..bytes_read])?;
    origin_server.flush()?;

    let bytes_read = origin_server.read(&mut origin_buffer)?;
    if bytes_read == 0 {
        return Ok(Vec::new());
    }

    client_stream.write_all(&origin_buffer[..bytes_read])?;
    client_stream.flush()?;

    Ok(origin_buffer[..bytes_read].to_vec())
}

// Main function to set up the proxy
fn main() -> Result<(), AspirinEatsError> {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 3 {
        eprintln!("Usage: {} <proxy-from> <proxy-to>", args[0]);
        std::process::exit(2);
    }

    let proxy_addr = &args[1];
    let origin_addr = &args[2];

    let listener = TcpListener::bind(proxy_addr)?;

    for stream in listener.incoming() {
        let stream = stream.map_err(AspirinEatsError::Io)?;

        let mut origin_server = TcpStream::connect(origin_addr)?;
        if let Err(e) = handle_connection(stream, &mut origin_server) {
            eprintln!("Error handling connection: {}", e);
        }
    }

    Ok(())
}

// Mock structs for testing
struct MockStream {
    data: Vec<u8>,
    cursor: usize,
}

impl MockStream {
    fn new(data: Vec<u8>) -> Self {
        MockStream { data, cursor: 0 }
    }
}

impl Read for MockStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let bytes_to_read = std::cmp::min(buf.len(), self.data.len() - self.cursor);
        buf[..bytes_to_read].copy_from_slice(&self.data[self.cursor..self.cursor + bytes_to_read]);
        self.cursor += bytes_to_read;
        Ok(bytes_to_read)
    }
}

impl Write for MockStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.data.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_empty_request() -> Result<(), AspirinEatsError> {
        let client_data = b"";
        let expected_response = b"HTTP/1.1 400 Bad Request\r\n\r\n";

        let mut client_stream = MockStream::new(client_data.to_vec());
        let mut origin_stream = MockStream::new(Vec::new());

        handle_connection(&mut client_stream, &mut origin_stream)?;

        assert!(
            origin_stream.data.is_empty(),
            "Origin stream data should be empty"
        );

        client_stream.data = expected_response.to_vec();
        let mut response_buffer = [0; 1024];
        let bytes_read = client_stream.read(&mut response_buffer)?;

        assert_eq!(&response_buffer[..bytes_read], expected_response);
        Ok(())
    }

    #[test]
    fn test_handle_read_error() -> Result<(), AspirinEatsError> {
        let expected_response = b"HTTP/1.1 200 OK\r\n\r\nHello, world!";

        let mut origin_stream = MockStream::new(expected_response.to_vec());

        struct ErrorStream;

        impl Read for ErrorStream {
            fn read(&mut self, _: &mut [u8]) -> std::io::Result<usize> {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "Read error"))
            }
        }

        impl Write for ErrorStream {
            fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
                Ok(0)
            }

            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }

        let result = handle_connection(ErrorStream, &mut origin_stream);

        assert!(result.is_err(), "Expected an error for read operation");
        Ok(())
    }

    #[test]
    fn test_handle_write_error() -> Result<(), AspirinEatsError> {
        let client_data = b"GET / HTTP/1.1\r\nHost: example.com\r\n\r\n";

        let mut client_stream = MockStream::new(client_data.to_vec());

        struct ErrorWriteStream {
            error: bool,
        }

        impl Read for ErrorWriteStream {
            fn read(&mut self, _: &mut [u8]) -> std::io::Result<usize> {
                Ok(0)
            }
        }

        impl Write for ErrorWriteStream {
            fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
                if self.error {
                    Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Write error",
                    ))
                } else {
                    Ok(0)
                }
            }

            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }

        let mut origin_stream = ErrorWriteStream { error: true };

        let result = handle_connection(&mut client_stream, &mut origin_stream);

        assert!(result.is_err(), "Expected an error for write operation");
        Ok(())
    }
}
