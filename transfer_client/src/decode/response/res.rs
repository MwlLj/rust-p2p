use number_conv::array::u8arr;

#[macro_export]
macro_rules! decode_response {
    ($index:ident, $data:ident, $response:ident) => ({
        if $index % 2 == 0 {
            let mut number: u64 = 0;
            u8arr::u8arrTou64($data.as_slice(), &mut number);
            return (true, number);
        }
        if $index == 1 {
            $response.responseMode = match String::from_utf8($data) {
                Ok(s) => s,
                Err(_) => "".to_string()
            };
        } else if $index == 3 {
            $response.serverUuid = match String::from_utf8($data) {
                Ok(s) => s,
                Err(_) => "".to_string()
            };
        } else if $index == 5 {
            $response.result = match String::from_utf8($data) {
                Ok(s) => {
                    if let Ok(n) = s.parse() {
                        n
                    } else {
                        1
                    }
                },
                Err(_) => 1
            };
        } else if $index == 7 {
            $response.selfCommunicateUuid = match String::from_utf8($data) {
                Ok(s) => s,
                Err(_) => "".to_string()
            };
        } else if $index == 9 {
            $response.peerCommunicateUuid = match String::from_utf8($data) {
                Ok(s) => s,
                Err(_) => "".to_string()
            };
        } else if $index == 11 {
            $response.objectUuid = match String::from_utf8($data) {
                Ok(s) => s,
                Err(_) => "".to_string()
            };
        } else if $index == 13 {
            $response.peerResult = match String::from_utf8($data) {
                Ok(s) => s,
                Err(_) => "".to_string()
            };
        } else if $index == 15 {
            u8arr::u8arrTou64($data.as_slice(), &mut $response.packageIndex);
        } else if $index == 17 {
            u8arr::u8arrTou64($data.as_slice(), &mut $response.packageTotal);
        } else if $index == 19 {
            $response.data = $data;
        }
        if $index == 19 {
            return (false, 0);
        }
        if $index == 1 || $index == 3 || $index == 5 || $index == 7 || $index == 8 || $index == 9 {
            return (true, 1);
        } else {
            return (true, 4);
        }
    })
}
