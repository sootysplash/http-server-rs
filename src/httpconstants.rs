use std::time::SystemTime;

pub struct HttpConstants {
}

impl HttpConstants {
    
    pub fn get_current_formatted_date() -> String { 
        // let add_days = 0 * 86400;
        let total_seconds = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
            // + add_days
            ;
        return HttpConstants::get_formatted_date(total_seconds);
    }
    
    pub fn get_formatted_date(total_seconds : u64) -> String {
        
        let seconds = total_seconds % 60;
        let mut minutes = total_seconds / 60;
        let mut hours = minutes / 60;
        minutes %= 60;
        let mut days = 1 + hours / 24;
        let total_days = days - 1;
        hours %= 24;
        let mut months = 0;
        let mut years = 1970;
        let mut month_length = 0;
        while days > month_length {
            month_length = HttpConstants::get_month_length(months + 1, years);
            months = (months + 1) % 12;
            days -= month_length;
            month_length = HttpConstants::get_month_length(months + 1, years);
            if months == 0 {
                years += 1;
            }
        }
        
        let format_months = months + 1; // during calc we start at 0;
        let week_day = (3 // thursday (5) - 2
            + total_days) % 7;
        
        
        
        return format!("{}, {} {} {} {}:{}:{} {}", 
            HttpConstants::get_day_name(week_day),
            HttpConstants::buffer_number(days),
            HttpConstants::get_month_name(format_months),
            years,
            HttpConstants::buffer_number(hours),
            HttpConstants::buffer_number(minutes),
            HttpConstants::buffer_number(seconds),
            "UTC");
    }
    
    fn get_month_length(months : u64, year : u64) -> u64 {
        if months == 1 // january
            || months == 3 // march
            || months == 5 // may
            || months == 7 // july
            || months == 8 // august
            || months == 10 // october
            || months == 12 // december 
        {
            return 31;
        } else if months == 2 { // february
            
            if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {// leap year check
                return 29;
            }
            
            return 28;
        }
        
        return 30; // all the other months
    }
    
    fn get_month_name(month : u64) -> String {
        return (match month {
            1 => "Jan",
            2 => "Feb",
            3 => "Mar",
            4 => "Apr",
            5 => "May",
            6 => "Jun",
            7 => "Jul",
            8 => "Aug",
            9 => "Sep",
            10 => "Oct",
            11 => "Nov",
            12 => "Dec",
            _ => {
                panic!("Invalid month number: {}", month);
            }
        }).to_string()
    }
    
    fn get_day_name(week_day : u64) -> String {
        return (match week_day {
            0 => "Mon",
            1 => "Tue",
            2 => "Wed",
            3 => "Thu",
            4 => "Fri",
            5 => "Sat",
            6 => "Sun",
            _ => {
                panic!("Invalid day number: {}", week_day);
            }
        }).to_string()
    }
    
    fn buffer_number(number : u64) -> String {
        let format_number;
        if number <= 9 {
            format_number = String::from("0") + number.to_string().as_str();
        } else {
            format_number = number.to_string();
        }
        return format_number;
    }
    
    pub fn get_code_text(code : i32) -> &'static str {
        return match code {
                HTTP_OK => " OK",
                HTTP_CONTINUE => " Continue",
                HTTP_CREATED => " Created",
                HTTP_ACCEPTED => " Accepted",
                HTTP_NOT_AUTHORITATIVE => " Non-Authoritative Information",
                HTTP_NO_CONTENT => " No Content",
                HTTP_RESET => " Reset Content",
                HTTP_PARTIAL => " Partial Content",
                HTTP_MULT_CHOICE => " Multiple Choices",
                HTTP_MOVED_PERM => " Moved Permanently",
                HTTP_MOVED_TEMP => " Temporary Redirect",
                HTTP_SEE_OTHER => " See Other",
                HTTP_NOT_MODIFIED => " Not Modified",
                HTTP_USE_PROXY => " Use Proxy",
                HTTP_BAD_REQUEST => " Bad Request",
                HTTP_UNAUTHORIZED => " Unauthorized" ,
                HTTP_PAYMENT_REQUIRED => " Payment Required",
                HTTP_FORBIDDEN => " Forbidden",
                HTTP_NOT_FOUND => " Not Found",
                HTTP_BAD_METHOD => " Method Not Allowed",
                HTTP_NOT_ACCEPTABLE => " Not Acceptable",
                HTTP_PROXY_AUTH => " Proxy Authentication Required",
                HTTP_CLIENT_TIMEOUT => " Request Time-Out",
                HTTP_CONFLICT => " Conflict",
                HTTP_GONE => " Gone",
                HTTP_LENGTH_REQUIRED => " Length Required",
                HTTP_PRECON_FAILED => " Precondition Failed",
                HTTP_ENTITY_TOO_LARGE => " Request Entity Too Large",
                HTTP_REQ_TOO_LONG => " Request-URI Too Large",
                HTTP_UNSUPPORTED_TYPE => " Unsupported Media Type",
                HTTP_INTERNAL_ERROR => " Internal Server Error",
                HTTP_NOT_IMPLEMENTED => " Not Implemented",
                HTTP_BAD_GATEWAY => " Bad Gateway",
                HTTP_UNAVAILABLE => " Service Unavailable",
                HTTP_GATEWAY_TIMEOUT => " Gateway Timeout",
                HTTP_VERSION => " HTTP Version Not Supported",
                _ => " ",
            }
    }
    
}

pub const HTTP_CONTINUE : i32 = 100;
pub const HTTP_OK : i32 = 200;
pub const HTTP_CREATED : i32 = 201;
pub const HTTP_ACCEPTED : i32 = 202;
pub const HTTP_NOT_AUTHORITATIVE : i32 = 203;
pub const HTTP_NO_CONTENT : i32 = 204;
pub const HTTP_RESET : i32 = 205;
pub const HTTP_PARTIAL : i32 = 206;
pub const HTTP_MULT_CHOICE : i32 = 300;
pub const HTTP_MOVED_PERM : i32 = 301;
pub const HTTP_MOVED_TEMP : i32 = 302;
pub const HTTP_SEE_OTHER : i32 = 303;
pub const HTTP_NOT_MODIFIED : i32 = 304;
pub const HTTP_USE_PROXY : i32 = 305;
pub const HTTP_BAD_REQUEST : i32 = 400;
pub const HTTP_UNAUTHORIZED : i32 = 401;
pub const HTTP_PAYMENT_REQUIRED : i32 = 402;
pub const HTTP_FORBIDDEN : i32 = 403;
pub const HTTP_NOT_FOUND : i32 = 404;
pub const HTTP_BAD_METHOD : i32 = 405;
pub const HTTP_NOT_ACCEPTABLE : i32 = 406;
pub const HTTP_PROXY_AUTH : i32 = 407;
pub const HTTP_CLIENT_TIMEOUT : i32 = 408;
pub const HTTP_CONFLICT : i32 = 409;
pub const HTTP_GONE : i32 = 410;
pub const HTTP_LENGTH_REQUIRED : i32 = 411;
pub const HTTP_PRECON_FAILED : i32 = 412;
pub const HTTP_ENTITY_TOO_LARGE : i32 = 413;
pub const HTTP_REQ_TOO_LONG : i32 = 414;
pub const HTTP_UNSUPPORTED_TYPE : i32 = 415;
pub const HTTP_INTERNAL_ERROR : i32 = 500;
pub const HTTP_NOT_IMPLEMENTED : i32 = 501;
pub const HTTP_BAD_GATEWAY : i32 = 502;
pub const HTTP_UNAVAILABLE : i32 = 503;
pub const HTTP_GATEWAY_TIMEOUT : i32 = 504;
pub const HTTP_VERSION : i32 = 505;
