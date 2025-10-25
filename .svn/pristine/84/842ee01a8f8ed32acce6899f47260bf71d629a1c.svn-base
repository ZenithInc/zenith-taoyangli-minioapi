<?php

use App\Core\Common\Handler\WriteLog;
use Hyperf\Contract\StdoutLoggerInterface;
use Hyperf\Utils\ApplicationContext;
use Hyperf\HttpServer\Contract\RequestInterface;

/**
 * 日期格式转化时间戳
 *
 * @param  string  $date  日期字符串
 * @param  string  $dateformat  ，需要标注当前日期格式
 *
 * @return number
 */
function getTimestamp($date, $dateformat = "Y-m-d H:i:s")
{
    if ($dateformat == "Y-m-d H:i:s") {
        $ymdhis = explode(" ", $date);
        $ymd = explode("-", $ymdhis[0]);
        $his = explode(":", $ymdhis[1]);

        return mktime($his[0], $his[1], $his[2], $ymd[1], $ymd[2], $ymd[0]);
    } elseif ($dateformat == "Y-m-d") {
        $ymd = explode("-", $date);

        return mktime(0, 0, 0, $ymd[1], $ymd[2], $ymd[0]);
    } elseif ($dateformat == "H:i:s") {
        $y = date("Y");
        $m = date("m");
        $d = date("d");
        $his = explode(":", $date);

        return mktime($his[0], $his[1], $his[2], $m, $d, $y);
    } else {
        $y = date("Y");
        $m = date("m");
        $d = date("d");
        $h = date("h");
        $i = date("i");
        $s = date("s");

        return mktime($h, $i, $s, $m, $d, $y);
    }
}

/**
 * 判断数组的键是否存在，并且不为空
 *
 * @param $arr
 * @param $column
 *
 * @return null
 */
function issetAndNotEmpty($arr, $column)
{
    return (isset($arr[$column]) && $arr[$column]) ? $arr[$column] : '';
}

/**
 * 除法运算
 *
 * @param  number  $num1
 * @param  number  $num2
 * @param  number  $len
 *
 * @return string
 */
function getBcdiv($num1, $num2, $len = 2)
{
    $str = function_exists("bcdiv") ? bcdiv($num1, $num2, $len * 10) : ($num1 / $num2);

    return number_format($str, $len, ".", "");
}

/**
 * 余运算
 *
 * @param  number  $num1
 * @param  number  $num2
 * @param  number  $len
 *
 * @return string
 */
function getBcmod($num1, $num2, $len = 2)
{
    $str = function_exists("bcmod") ? bcmod($num1, $num2, $len * 10) : ($num1 % $num2);

    return number_format($str, $len, ".", "");
}

/**
 * 求幂
 *
 * @param  number  $num1
 * @param  number  $num2
 * @param  number  $len
 *
 * @return string
 */
function getBcpow($num1, $num2, $len = 2)
{
    $str = function_exists("bcpow") ? bcpow($num1, $num2, $len * 10) : pow($num1, $num2);

    return number_format($str, $len, ".", "");
}

/**
 * 乘运算
 *
 * @param  number  $num1
 * @param  number  $num2
 * @param  number  $len
 *
 * @return string
 */
function getBcmul($num1, $num2, $len = 2)
{
    $str = function_exists("bcmul") ? bcmul($num1, $num2, $len * 10) : ($num1 * $num2);

    return number_format($str, $len, ".", "");
}

/**
 * 减运算
 *
 * @param  number  $num1
 * @param  number  $num2
 * @param  number  $len
 *
 * @return string
 */
function getBcsub($num1, $num2, $len = 2)
{
    $str = function_exists("bcsub") ? bcsub($num1, $num2, $len * 10) : ($num1 - $num2);

    return number_format($str, $len, ".", "");
}

/**
 * 加运算
 *
 * @param  number  $num1
 * @param  number  $num2
 * @param  number  $len
 *
 * @return string
 */
function getBcadd($num1, $num2, $len = 2)
{
    $str = function_exists("bcadd") ? bcadd($num1, $num2, $len * 10) : ($num1 + $num2);

    return number_format($str, $len, ".", "");
}

/**
 * 坐标距离计算
 *
 * @param  unknown  $lat1
 * @param  unknown  $lng1
 * @param  unknown  $lat2
 * @param  unknown  $lng2
 *
 * @return number
 */
function getDistance($lat1, $lng1, $lat2, $lng2)
{
    $EARTH_RADIUS = 6378.137;
    $radLat1 = getRad($lat1);
    $radLat2 = getRad($lat2);
    $a = $radLat1 - $radLat2;
    $b = getRad($lng1) - getRad($lng2);
    $s = 2 * asin(sqrt(pow(sin($a / 2), 2) + cos($radLat1) * cos($radLat2) * pow(sin($b / 2), 2)));
    $s = $s * $EARTH_RADIUS;
    $s = round($s * 10000) / 10000;

    return $s;
}

/*
 * 根据经纬度距离获取距离
 * @return $DistanceDiff 两点间距离米
 */
function getDistanceDiff($lat, $lng, $nowLat, $newLng)
{
    $earthRadius = 6367000;
    $lat = ((float)$lat * pi()) / 180;
    $lng = ((float)$lng * pi()) / 180;
    $nowLat = ($nowLat * pi()) / 180;
    $newLng = ($newLng * pi()) / 180;
    $calcLongitude = $newLng - $lng;
    $calcLatitude = $nowLat - $lat;
    $stepOne = pow(sin($calcLatitude / 2), 2) + cos($lat) * cos($nowLat) * pow(sin($calcLongitude / 2), 2);
    $stepTwo = 2 * asin(min(1, sqrt($stepOne)));
    $distanceDiff = $earthRadius * $stepTwo;

    return round($distanceDiff);
}

/**
 * 圆周率
 *
 * @param  number  $d
 *
 * @return number
 */
function getRad($d)
{
    return $d * 3.1415926535898 / 180.0;
}

/**
 * 设置签名
 *
 * @param  array  $param
 * @param  string  $key
 *
 * @return string
 */
function setSign($param, $key)
{
    $param = paramSort($param);
    //    file_put_contents(storage_path('logs/sign.log'), var_export($param, true) . "_1" . "\r\n", FILE_APPEND);
    $query = http_build_query($param);

    //    file_put_contents(storage_path('logs/sign.log'), $query . "_2" . "\r\n", FILE_APPEND);
    return md5(md5($query).$key);
}

/**
 * 参数排序
 *
 * @param $param
 *
 * @return mixed
 */
function paramSort($param)
{
    ksort($param, SORT_STRING);
    foreach ($param as $key => $value) {
        if (is_array($value)) {
            $param[$key] = paramSort($value);
        }
    }

    return $param;
}

/**
 * 会话加密方式
 *
 * @param  string  $string
 * @param  string  $operation
 * @param  string  $key
 *
 * @return string
 */
function fauthCode($string, $operation = 'ENCODE', $key = '')
{
    $ckey_length = 4; // 随机密钥长度 取值 0-32;
    // 加入随机密钥，可以令密文无任何规律，即便是原文和密钥完全相同，加密结果也会每次不同，增大破解难度。
    // 取值越大，密文变动规律越大，密文变化 = 16 的 $ckey_length 次方
    // 当此值为 0 时，则不产生随机密钥
    $string = isset($string) ? $string : '';
    $key = md5($key);
    $keya = md5(substr($key, 0, 16));
    $keyb = md5(substr($key, 16, 16));
    $keyc = $ckey_length ? ($operation == 'DECODE' ? substr($string, 0, $ckey_length) : substr(md5(microtime()),
        -$ckey_length)) : '';
    $cryptkey = $keya.md5($keya.$keyc);
    $key_length = strlen($cryptkey);
    $string = $operation == 'DECODE' ? base64_decode(substr($string, $ckey_length)) : sprintf('%010d',
            0).substr(md5($string.$keyb), 0, 16).$string;
    $string_length = strlen($string);
    $result = '';
    $box = range(0, 255);
    $rndkey = [];
    for ($i = 0; $i <= 255; $i++) {
        $rndkey[$i] = ord($cryptkey[$i % $key_length]);
    }
    for ($j = $i = 0; $i < 256; $i++) {
        $j = ($j + $box[$i] + $rndkey[$i]) % 256;
        $tmp = $box[$i];
        $box[$i] = $box[$j];
        $box[$j] = $tmp;
    }
    for ($a = $j = $i = 0; $i < $string_length; $i++) {
        $a = ($a + 1) % 256;
        $j = ($j + $box[$a]) % 256;
        $tmp = $box[$a];
        $box[$a] = $box[$j];
        $box[$j] = $tmp;
        $result .= chr(ord($string[$i]) ^ ($box[($box[$a] + $box[$j]) % 256]));
    }
    if ($operation == 'DECODE') {
        if ((substr($result, 0, 10) == 0 || substr($result, 0, 10) > 0)
            && substr($result, 10,
                16) == substr(md5(substr($result, 26).$keyb), 0, 16)) {
            return substr($result, 26);
        } else {
            return '';
        }
    } else {
        return $keyc.str_replace('=', '', base64_encode($result));
    }
}
/**
 * 会话加密方式
 * @param string $string
 * @param string $operation
 * @param string $key
 * @return string
 */
function oldFauthCode($string,$operation='ENCODE',$key=''){
    $ckey_length=4; // 随机密钥长度 取值 0-32;
    // 加入随机密钥，可以令密文无任何规律，即便是原文和密钥完全相同，加密结果也会每次不同，增大破解难度。
    // 取值越大，密文变动规律越大，密文变化 = 16 的 $ckey_length 次方
    // 当此值为 0 时，则不产生随机密钥
    $string=isset($string)?$string:'';
    $key=md5($key);
    $keya=md5(substr($key,0,16));
    $keyb=md5(substr($key,16,16));
    $keyc=$ckey_length?($operation=='DECODE'?substr($string,0,$ckey_length):substr(md5(microtime()),-$ckey_length)):'';
    $cryptkey=$keya.md5($keya.$keyc);
    $key_length=strlen($cryptkey);
    $string=$operation=='DECODE'?base64_decode(substr($string,$ckey_length)):sprintf('%010d',0).substr(md5($string.$keyb),0,16).$string;
    $string_length=strlen($string);
    $result='';
    $box=range(0,255);
    $rndkey=array();
    for($i=0;$i<=255;$i++){
        $rndkey[$i]=ord($cryptkey[$i%$key_length]);
    }
    for($j=$i=0;$i<256;$i++){
        $j=($j+$box[$i]+$rndkey[$i])%256;
        $tmp=$box[$i];
        $box[$i]=$box[$j];
        $box[$j]=$tmp;
    }
    for($a=$j=$i=0;$i<$string_length;$i++){
        $a=($a+1)%256;
        $j=($j+$box[$a])%256;
        $tmp=$box[$a];
        $box[$a]=$box[$j];
        $box[$j]=$tmp;
        $result.=chr(ord($string[$i])^($box[($box[$a]+$box[$j])%256]));
    }
    if($operation=='DECODE'){
        if((substr($result,0,10)==0||substr($result,0,10)>0)&&substr($result,10,16)==substr(md5(substr($result,26).$keyb),0,16)){
            return substr($result,26);
        }else{
            return '';
        }
    }else{
        return $keyc.str_replace('=','',base64_encode($result));
    }
}
function getBrowser()
{
    if (empty($_SERVER['HTTP_USER_AGENT'])) {
        return '';
    }
    $agent = $_SERVER['HTTP_USER_AGENT'];
    $browser = '';
    $browser_ver = '';
    if (preg_match('/MSIE\s([^\s|;]+)/i', $agent, $regs)) {
        if (preg_match('/Maxthon/i', $agent, $regs)) {
            $browser = 'Maxthon (IE '.$browser_ver.')';
        } elseif (preg_match('/360SE/i', $agent, $regs)) {
            $browser = '360 (IE '.$browser_ver.')';
        } elseif (preg_match('/NetCaptor\s([^\s|;]+)/i', $agent, $regs)) {
            $browser = 'NetCaptor (IE '.$browser_ver.')';
        } else {
            $browser = 'Internet Explorer';
        }
        $browser_ver = empty($regs[1]) ? '' : $regs[1];
    } elseif (preg_match('/FireFox\/([^\s]+)/i', $agent, $regs)) {
        $browser = 'FireFox';
        $browser_ver = $regs[1];
    } elseif (preg_match('/Opera[\s|\/]([^\s]+)/i', $agent, $regs)) {
        $browser = 'Opera';
        $browser_ver = $regs[1];
    } elseif (preg_match('/OmniWeb\/(v*)([^\s|;]+)/i', $agent, $regs)) {
        $browser = 'OmniWeb';
        $browser_ver = $regs[2];
    } elseif (preg_match('/Netscape([\d]*)\/([^\s]+)/i', $agent, $regs)) {
        $browser = 'Netscape';
        $browser_ver = $regs[2];
    } elseif (preg_match('/Chrome\/([^\s]+)/i', $agent, $regs)) {
        $browser = 'Chrome ('.$regs[1].')';
        $browser_ver = $regs[1];
    } elseif (preg_match('/safari\/([^\s]+)/i', $agent, $regs)) {
        $browser = 'Safari';
        $browser_ver = $regs[1];
    } elseif (preg_match('/Lynx\/([^\s]+)/i', $agent, $regs)) {
        $browser = 'Lynx';
        $browser_ver = $regs[1];
    } elseif (preg_match('/iPhone\/([^\s]+)/i', $agent, $regs)) {
        $browser = 'iPhone ('.$regs[1].')';
        $browser_ver = $regs[1];
    } elseif (preg_match('/Android\/([^\s]+)/i', $agent, $regs)) {
        $browser = 'Android ('.$regs[1].')';
        $browser_ver = $regs[1];
    }
    if ( !empty($browser)) {
        return empty($browser_ver) ? addslashes($browser) : addslashes($browser.' '.$browser_ver);
    } else {
        return 'Unknow browser';
    }
}

/**
 * 获得客户端的操作系统
 *
 * @access private
 * @return void
 */
function getOs()
{
    if (empty($_SERVER['HTTP_USER_AGENT'])) {
        return 'Unknown';
    }
    $agent = strtolower($_SERVER['HTTP_USER_AGENT']);
    $os = '';
    if (strpos($agent, 'win') !== false) {
        if (strpos($agent, 'nt 5.1') !== false) {
            $os = 'Windows XP';
        } elseif (strpos($agent, 'nt 5.2') !== false) {
            $os = 'Windows 2003';
        } elseif (strpos($agent, 'nt 5.0') !== false) {
            $os = 'Windows 2000';
        } elseif (strpos($agent, 'nt 6.0') !== false) {
            $os = 'Windows Vista';
        } elseif (strpos($agent, 'nt 6.1') !== false) {
            $os = 'Windows 7';
        } elseif (strpos($agent, 'nt') !== false) {
            $os = 'Windows NT';
        } elseif (strpos($agent, 'win 9x') !== false && strpos($agent, '4.90') !== false) {
            $os = 'Windows ME';
        } elseif (strpos($agent, '98') !== false) {
            $os = 'Windows 98';
        } elseif (strpos($agent, '95') !== false) {
            $os = 'Windows 95';
        } elseif (strpos($agent, '32') !== false) {
            $os = 'Windows 32';
        } elseif (strpos($agent, 'ce') !== false) {
            $os = 'Windows CE';
        }
    } elseif (strpos($agent, 'linux') !== false) {
        $os = 'Linux';
    } elseif (strpos($agent, 'unix') !== false) {
        $os = 'Unix';
    } elseif (strpos($agent, 'sun') !== false && strpos($agent, 'os') !== false) {
        $os = 'SunOS';
    } elseif (strpos($agent, 'ibm') !== false && strpos($agent, 'os') !== false) {
        $os = 'IBM OS/2';
    } elseif (strpos($agent, 'mac') !== false && strpos($agent, 'pc') !== false) {
        $os = 'Macintosh';
    } elseif (strpos($agent, 'powerpc') !== false) {
        $os = 'PowerPC';
    } elseif (strpos($agent, 'aix') !== false) {
        $os = 'AIX';
    } elseif (strpos($agent, 'hpux') !== false) {
        $os = 'HPUX';
    } elseif (strpos($agent, 'netbsd') !== false) {
        $os = 'NetBSD';
    } elseif (strpos($agent, 'bsd') !== false) {
        $os = 'BSD';
    } elseif (strpos($agent, ' osf1') !== false) {
        $os = 'OSF1';
    } elseif (strpos($agent, 'irix') !== false) {
        $os = 'IRIX';
    } elseif (strpos($agent, 'freebsd') !== false) {
        $os = 'FreeBSD';
    } elseif (strpos($agent, 'teleport') !== false) {
        $os = 'teleport';
    } elseif (strpos($agent, 'flashget') !== false) {
        $os = 'flashget';
    } elseif (strpos($agent, 'webzip') !== false) {
        $os = 'webzip';
    } elseif (strpos($agent, 'offline') !== false) {
        $os = 'offline';
    } elseif (strpos($agent, 'iPhone') !== false) {
        $os = 'iPhone';
    } elseif (strpos($agent, 'Android') !== false) {
        $os = 'Android';
    } else {
        $os = 'Unknown';
    }

    return $os;
}

function hasPermission($role, $module)
{
    /* $roleConfig = config('roles');
    if (empty($role) || empty($roleConfig) || !in_array($role, ['1', '2', '3', '4'])) {
        return [];
    }
    $roleArr = $roleConfig[$role];
    return $roleArr; */
    $roleConfig = config('roles');
    if (empty($role) || !in_array($module, ['Manage', 'console'])) {
        return [];
    }
    if ( !is_array($role)) {
        $role = explode(',', $role);
    }
    $roleArr = $roleConfig[$module];
    $roles = [];
    foreach ($role as $value) {
        if (isset($roleArr[$value])) {
            $roles[$value] = $roleArr[$value];
        }
    }

    return $roles;
}

/** utf82Gb 转换utf8字符到gbk
 *
 * @param  string  $gbstr
 *
 * @return string 转换后的字符串
 */
function utf82Gbk($utfStr)
{
    if (function_exists('mb_convert_encoding')) {
        return mb_convert_encoding($utfStr, 'GBK', 'utf-8');
    } else {
        if (function_exists('iconv')) {
            return iconv("utf-8", "GBK", $utfStr);
        }
    }

    return $utfStr;
}

/**
 * 获取数据
 *
 * @param  unknown  $url
 * @param  string  $body
 * @param  string  $method
 * @param  unknown  $headers
 *
 * @return multitype:mixed string
 */
function curlHttp($url = '', $body = '', $method = 'DELETE', $headers = [])
{
    $httpinfo = [];
    $curl = curl_init();
    if (preg_match('/^(.*?):(\d+)$/si', $url, $match)) {
        curl_setopt($curl, CURLOPT_URL, $match[1]);
        curl_setopt($curl, CURLOPT_PORT, $match[2]);
    } else {
        curl_setopt($curl, CURLOPT_URL, $url);
    }
    curl_setopt($curl, CURLOPT_HTTP_VERSION, CURL_HTTP_VERSION_1_0);
    if (stripos($url, "https://") !== false) {
        if (defined('CURL_SSLVERSION_TLSv1')) {
            curl_setopt($curl, CURLOPT_SSLVERSION, CURL_SSLVERSION_TLSv1);
        }
        curl_setopt($curl, CURLOPT_SSL_VERIFYPEER, false); //不验证证书
        curl_setopt($curl, CURLOPT_SSL_VERIFYHOST, false); //不验证证书
    }
    curl_setopt($curl, CURLOPT_USERAGENT, 'developer.douya.wang');
    curl_setopt($curl, CURLOPT_RETURNTRANSFER, true);
    curl_setopt($curl, CURLOPT_CONNECTTIMEOUT, 30);
    curl_setopt($curl, CURLOPT_TIMEOUT, 30);
    curl_setopt($curl, CURLINFO_HEADER_OUT, true);
    if (empty($headers)) {
        curl_setopt($curl, CURLOPT_HEADER, false);
    } else {
        curl_setopt($curl, CURLOPT_HTTPHEADER, $headers);
    }
    switch ($method) {
        case 'POST':
            curl_setopt($curl, CURLOPT_POST, true);
            if ( !empty($body)) {
                if (is_array($body)) {
                    curl_setopt($curl, CURLOPT_POSTFIELDS, http_build_query($body));
                } else {
                    curl_setopt($curl, CURLOPT_POSTFIELDS, $body);
                }
            }
            break;
        case 'FILE':
            curl_setopt($curl, CURLOPT_POST, true);
            curl_setopt($curl, CURLOPT_SAFE_UPLOAD, false);
            curl_setopt($curl, CURLOPT_POSTFIELDS, $body);
            break;
        case 'DELETE':
            curl_setopt($curl, CURLOPT_CUSTOMREQUEST, 'GET');
            if ( !empty($body)) {
                $url = $url.'?'.http_build_query($body);
            }
            break;
    }
    $httpinfo['response'] = curl_exec($curl);
    $httpinfo['code'] = curl_getinfo($curl, CURLINFO_HTTP_CODE);
    $httpinfo['error'] = curl_error($curl);
    curl_close($curl);

    return $httpinfo;
}
function curlHttp1($url = '', $body = '', $method = 'DELETE', $headers = [])
{
    $tempHeaders = [];
    if($headers){
        foreach ($headers as $v){
            $arr = explode(':',$v);
            $tempHeaders[$arr[0]] = $arr[1] ?? '';
        }
    }
    // 创建 GuzzleHttp 客户端实例
    $client = new \GuzzleHttp\Client();

    // 根据请求方法和参数创建 GuzzleHttp 请求实例
    $requestOptions = [];
    switch ($method) {
        case 'POST':
            $requestOptions['form_params'] = $body;
            break;
        case 'FILE':
            $requestOptions['multipart'] = $body;
            break;
        case 'DELETE':
            $requestOptions['query'] = $body;
            break;
    }
    if (!empty($tempHeaders)) {
        $requestOptions['headers'] = $tempHeaders;
    }

    // 发送请求并获取响应
    try {
        $response = $client->request($method, $url, $requestOptions);
        $httpinfo['response'] = $response->getBody()->getContents();
        $httpinfo['code'] = $response->getStatusCode();
        $httpinfo['error'] = '';
    } catch (\GuzzleHttp\Exception\ClientException $e) {
        $response = $e->getResponse();
        $httpinfo['response'] = $response->getBody()->getContents();
        $httpinfo['code'] = $response->getStatusCode();
        $httpinfo['error'] = $e->getMessage();
    }

    return $httpinfo;
}
/**
 * 随机生成
 *
 * @param  int  $leng
 * @param  string  $type
 *
 * @return string
 */
function randomCode($leng = 12, $type = 'all')
{
    $leng = $leng > 0 ? intval($leng) : 12;
    $str = '';
    if ($type == 'num') {
        for ($i = 0; $i < $leng; $i++) {
            $str .= chr(mt_rand(48, 57));
        }
    } elseif ($type == 'capital') {
        for ($i = 0; $i < $leng; $i++) {
            $str .= chr(mt_rand(65, 90));
        }
    } elseif ($type == 'lower') {
        for ($i = 0; $i < $leng; $i++) {
            $str .= chr(mt_rand(97, 122));
        }
    } else {
        $char = [
            "1",
            "2",
            "3",
            "4",
            "5",
            "6",
            "7",
            "8",
            "9",
            "0",
            "a",
            "b",
            "c",
            "d",
            "e",
            "f",
            "g",
            "h",
            "i",
            "j",
            "k",
            "l",
            "m",
            "n",
            "o",
            "p",
            "q",
            "r",
            "s",
            "t",
            "u",
            "v",
            "w",
            "x",
            "y",
            "z",
            "A",
            "B",
            "C",
            "D",
            "E",
            "F",
            "G",
            "H",
            "I",
            "J",
            "K",
            "L",
            "M",
            "N",
            "O",
            "P",
            "Q",
            "R",
            "S",
            "T",
            "U",
            "V",
            "W",
            "X",
            "Y",
            "Z",
        ];
        shuffle($char);
        $n = count($char) - 1;
        $str = '';
        for ($i = 0; $i < $leng; $i++) {
            $m = mt_rand(0, $n);
            $str .= $char[$m];
        }
    }

    return $str;
}

/**
 * 十进制数字转换为52进制字母
 *
 * @param $num
 *
 * @return bool|string
 */
function num2Letter($num)
{
    $num = intval($num);
    if ($num <= 0) {
        return false;
    }
    $letterArr = [
        'a',
        'b',
        'c',
        'd',
        'e',
        'f',
        'g',
        'h',
        'i',
        'j',
        'k',
        'l',
        'm',
        'n',
        'o',
        'p',
        'q',
        'r',
        's',
        't',
        'u',
        'v',
        'w',
        'x',
        'y',
        'z',
        'A',
        'B',
        'C',
        'D',
        'E',
        'F',
        'G',
        'H',
        'I',
        'J',
        'K',
        'L',
        'M',
        'N',
        'O',
        'P',
        'Q',
        'R',
        'S',
        'T',
        'U',
        'V',
        'W',
        'X',
        'Y',
        'Z',
    ];
    $letter = '';
    do {
        $key = ($num - 1) % 52;
        $letter = $letterArr[$key].$letter;
        $num = floor(($num - $key) / 52);
    } while ($num > 0);

    return $letter;
}

/**
 * 52进制字母转换为十进制数字
 *
 * @param $str
 *
 * @return float|int|string
 */
function letter2Num($str)
{
    $str = preg_replace("/[0-9]/", "", $str);
    $transfArray = [
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
    ];
    $len = strlen($str);
    $letter = 0;
    for ($i = $len - 1; $i >= 0; $i--) {
        $letter += $transfArray[$str[$i]] * pow(52, $len - 1 - $i);
    }

    return $letter;
}

function randomStr($str, $leng = 12)
{
    $leng = $leng > 0 ? intval($leng) : 12;
    $leng = $leng - strlen($str);
    $newStr = "";
    for ($i = 0; $i < $leng; $i++) {
        if (empty($str[$i])) {
            $newStr .= chr(mt_rand(48, 57));
        } else {
            $newStr .= $str[$i].chr(mt_rand(48, 57));
        }
    }
    for ($i = 0; $i < strlen($str) - $leng; $i++) {
        $newStr .= $str[$i];
    }

    return $newStr;
}

function getDriveSign($param)
{
    $key = config('driveapi.secret_key');
    if (empty($param['appid'])) {
        return '缺少appid';
    }
    $appid = $param['appid'];
    $timeStamp = $param['timeStamp'];
    $param = paramSort($param);
    $query = http_build_query($param);

    return md5($appid.$timeStamp.md5($query).$key);
}

function driveSignVerify()
{
    $param = request()->all();
    $sign = '';
    foreach ($param as $k => $v) {
        if (strpos($k, '/') !== false) {
            unset($param[$k]);
        }
        if ($k == 'sign') {
            $sign = $v;
            unset($param[$k]);
        }
    }
    unset($v);
    $param['appid'] = config('driveapi.appid');
    $driveSign = getDriveSign($param);
    if ($sign !== $driveSign) {
        return false;
    }

    return true;
}

/**
 * 获取日期段的每一日
 *
 * @param $startDate
 * @param $endDate
 *
 * @return array
 */
function getDateAll($startDate, $endDate)
{
    return array_map(function ($n) {
        return date('Y-m-d', $n);
    }, range(strtotime($startDate), strtotime($endDate), 24 * 3600));
}

/**
 * 获取周几
 *
 * @param $date
 *
 * @return string
 */
function getWeek($date)
{
    $weekArr = ["日", "一", "二", "三", "四", "五", "六"];
    $week = date("w", $date);

    return empty($weekArr[$week]) ? '' : '周'.$weekArr[$week];
}

/**
 * 手机验证
 *
 * @param $mobile
 *
 * @return bool
 */
function checkMobile($mobile)
{
    if ($mobile) {
        if (preg_match('/^1[3456789]\d{9}$/', $mobile)) {
            return true;
        }
    }

    return false;
}

/**
 * 座机验证
 *
 * @param $tel
 *
 * @return bool
 */
function checkTel($tel)
{
    if ($tel) {
        if (preg_match('/^([0-9]{3,4}-)?[0-9]{7,8}$/', $tel)) {
            return true;
        }
    }

    return false;
}

/**
 * 生成订单号
 *
 * @param  int  $prefix
 * @param  string  $suffix
 *
 * @return string
 */
function createTradeSn($prefix = 2, $suffix = '', $uid = 0)
{
    //    mt_srand((double ) microtime() * 1000000);
    if (empty($uid)) {
        $uid = mt_rand(1, 10000000);
    }
    $leng = 4;
    $padUid = str_pad(substr($uid, 0, $leng), $leng, '0', STR_PAD_LEFT);
    $randNum = str_pad(mt_rand(1, 100), 3, '0', STR_PAD_LEFT);
    //        file_put_contents('/Users/code/test/rn.txt', $prefix . date("YmdHis") . str_pad(mt_rand(1, 100),3,'0',STR_PAD_LEFT) . $padUid . $suffix.',',FILE_APPEND);
    //        die();
    return $prefix.date("YmdHis").$padUid.$randNum.$suffix;
}

/**
 * 获取平台活动订单状态
 *
 * @param $status
 *
 * @return mixed|string
 */
function getFestivalsOrderStatus($status)
{
    $statusArr = ['其他', '待支付', '已取消', '支付中', '待核销', '已核销', '退款中', '已退款'];
    return empty($statusArr[$status]) ? '' : $statusArr[$status];
}

/**
 * 获取性别
 *
 * @param $gender
 *
 * @return mixed|string
 */
function getGender($gender)
{
    $genderArr = [
        'secrecy' => '保密',
        'male'    => '男',
        'female'  => '女',
    ];

    return empty($genderArr[$gender]) ? '' : $genderArr[$gender];
}

/**
 * 10位扫码字符串, 十进制数字转换为26进制字母
 *
 * @param  int  $id
 *
 * @return string
 */
function scanCodeCreate($id, $leng = 10)
{
    $letter = '';
    if ( !empty($id)) {
        $letter = num2Letter($id);
        $letter = randomStr($letter, $leng);
    }

    return $letter;
}

/**
 * 脱敏操作
 *
 * @param $string string 待处理的字符串
 * @param  int  $start  规定在字符串的何处开始，
 * @param  int  $length  可选。规定要隐藏的字符串长度。
 * @param  string  $re  替代符
 *
 * @return bool|string 处理后的字符串
 */
function formatStr($string, $start = 0, $length = 0, $re = '*')
{
    $strLen = empty($string) ? '' : mb_strlen($string);
    if (empty($strLen)) {
        return $string;
    }
    $length = empty($length) ? $strLen : $length;
    $re = empty($re) ? '*' : $re;
    $end = $start + $length;
    $strArr = [];
    for ($i = 0; $i < $strLen; $i++) {
        if ($i >= $start && $i < $end) {
            $strArr[] = $re;
        } else {
            $strArr[] = mb_substr($string, $i, 1);
        }
    }

    return implode('', $strArr);
}

/**
 * 6位短信核销码, 十进制数字转换为52进制字母
 *
 * @param  int  $id
 *
 * @return string
 */
function smsCodeCreate($id, $leng = 10)
{
    $letter = '';
    if ( !empty($id)) {
        $letter = num2Letter($id);
        $letter = randomStr($letter, $leng);
    }

    return $letter;
}

/**
 * 输出订单ID, 由核销码字符串反解析成
 *
 * @param $code
 *
 * @return string|string[]|null
 */
function codeDecrypt($code)
{
    $id = '';
    if ( !empty($code)) {
        $id = letter2Num($code);
    }

    return $id;
}

/*wg
 * 二维数组排序，将其作为一个表的列并以行来进行排序
 * @param $arrays      必填 待排序二维数组
 * @param $sort_key    必填 作为排序依据的第二纬的键名
 * @param $sort_order  可选 排列顺序, 可能的值是 SORT_ASC 和 SORT_DESC
 * @param $sort_type   可选。规定排序类型。可能的值是SORT_REGULAR、SORT_NUMERIC（将每一项按数字顺序排列）和SORT_STRING（将每一项按字母顺序排列）。
 * return array
 */
function myMultisort($arrays, $sort_key, $sort_order = SORT_ASC, $sort_type = SORT_NUMERIC)
{
    if ( !empty($arrays) ?? is_array($arrays)) {
        foreach ($arrays as $array) {
            if (is_array($array)) {
                $key_arrays[] = $array[$sort_key];
            } else {
                return [];
            }
        }
    } else {
        return [];
    }
    array_multisort($key_arrays, $sort_order, $sort_type, $arrays);

    return $arrays;
}

/**
 * wcz
 * 获取图片描述exif
 *
 * @param $url
 *
 * @return array
 */
function getPictureExif($url)
{
    if (empty($url)) {
        return [];
    }
    try {
        $url = sprintf("%s!exif", $url);
        $url = getTokenPicture($url, 0, 1);
        $return = curlHttp($url);
        if ($return['code'] == 200) {
            $response = json_decode($return['response'], true);
            if ( !empty($response)) {
                $gpsLongitude = '';
                $gpsLatitude = '';
                $digitizedTime = !empty($response['EXIF']['DateTimeDigitized']) ? strtotime($response['EXIF']['DateTimeDigitized']) : '';
                if ( !empty($response['EXIF']['GPSLongitude']) && !empty($response['EXIF']['GPSLatitude'])) {
                    $gpsLocation = getGpsToCoordinate($response['EXIF']['GPSLongitude'], $response['EXIF']['GPSLatitude']);
                    $gpsLongitude = $gpsLocation['lng'] ?? '';
                    $gpsLatitude = $gpsLocation['lat'] ?? '';
                }

                return [
                    'width'         => $response['width'] ?? 0, //尺寸,宽
                    'height'        => $response['height'] ?? 0, //尺寸,高
                    'model'         => $response['EXIF']['Model'] ?? '',//型号
                    'iso'           => $response['EXIF']['ISOSpeedRatings'] ?? '',//ISO
                    'focal'         => $response['EXIF']['FocalLength'] ?? '',//焦距
                    'exposure'      => $response['EXIF']['ExposureBiasValue'] ?? '',//曝光补偿
                    'fnumber'       => $response['EXIF']['FNumber'] ?? '',//光圈
                    'aperture'      => $response['EXIF']['ApertureValue'] ?? '',//镜头
                    'shutter'       => $response['EXIF']['ShutterSpeedValue'] ?? '',//快门速度
                    'time'          => $digitizedTime,//拍摄时间
                    'gps_longitude' => $gpsLongitude,//拍摄位置，经度
                    'gps_latitude'  => $gpsLatitude,//拍摄位置，纬度
                ];
            }
        }
    } catch (Exception $e) {
        $msg = $e->getMessage();
        $line = $e->getFile().':'.$e->getLine();
        WriteLog::error(sprintf('智能获取图片[%s]exif描述失败, %s %s', $url, $line, $msg));
    }

    return [];
}

/**
 * wcz
 * 获取图片尺寸选择列表
 *
 * @param  array  $imageSize  原图尺寸 ["宽尺寸","高尺寸"]
 *
 * @return array
 */
function getBuySize(array $imageSize)
{
    $sizeList = [];
    if (empty($imageSize[0]) || empty($imageSize[1])) {
        return $sizeList;
    }
    //原图宽高
    $oriWidth = (int) $imageSize[0] ?? 0;
    $oriHeight = (int) $imageSize[1] ?? 0;
    $oriPer = floatval(getBcdiv($oriWidth, $oriHeight, 6));//宽高比
    $sizeSet = config('picture.sizeSet');
    if ( !empty($sizeSet['shortest']['enabled'])) {
        //最短边原则
        $sizes = myMultisort($sizeSet['shortest']['sizes'] ?? [], 'ori', SORT_DESC);
        foreach ($sizes as &$size) {
            //最短尺寸, 按照宽高的最短边来获取新的尺寸
            $minSize = $size['min'] ?? 0;
            if ($oriWidth > $oriHeight) {
                if ($minSize < $oriHeight) {
                    $height = $minSize;
                    $width = round(getBcmul($height, $oriPer));
                } else {
                    $width = $oriWidth;
                    $height = $oriHeight;
                }
                $oriMinSize = $oriHeight;
            } else {
                if ($minSize < $oriWidth) {
                    $width = $minSize;
                    $height = round(getBcdiv($width, $oriPer));
                } else {
                    $width = $oriWidth;
                    $height = $oriHeight;
                }
                $oriMinSize = $oriWidth;
            }
            //原图
            if ($size['ori'] == 1) {
                $sizeList[] = [
                    'min'   => $oriWidth,
                    'max'   => $oriHeight,
                    'price' => $size['price'] ?? 0,
                    'ori'   => $size['ori'] ?? 0,
                ];
            } elseif ($minSize <= $oriMinSize) {
                $sizeList[] = [
                    'min'   => $width,
                    'max'   => $height,
                    'price' => $size['price'] ?? 0,
                    'ori'   => $size['ori'] ?? 0,
                ];
            } else {
                $sizeList[] = [
                    'min'   => $width,
                    'max'   => $height,
                    'price' => $size['price'] ?? 0,
                    'ori'   => $size['ori'] ?? 0,
                ];
                break;
            }
        }
        unset($size);
    }
    if ( !empty($sizeSet['longest']['enabled'])) {
        //最长边原则
        $sizes = myMultisort($sizeSet['longest']['sizes'] ?? [], 'ori', SORT_DESC);
        foreach ($sizes as &$size) {
            //最长尺寸, 按照宽高的最长边来获取新的尺寸
            $maxSize = $size['max'] ?? 0;
            //原图的过滤掉尺寸
            if ($size['ori'] == 1 || ($maxSize > $oriWidth && $maxSize > $oriHeight)) {
                continue;
            }
            if ($oriWidth >= $oriHeight) {
                //1920 1500
                if ($maxSize <= $oriHeight) {
                    $width = $maxSize;
                    $height = round(getBcdiv($width, $oriPer));
                } else {
                    //2000 800
                    $width = $maxSize;
                    $height = round(getBcdiv($width, $oriPer));
                }
            } else {
                //   1000   1920
                if ($maxSize <= $oriHeight) {
                    $width = round(getBcmul($maxSize, $oriPer));
                    $height = $maxSize;
                } else {
                    //   800   2000
                    $width = round(getBcmul($oriWidth, $oriPer));
                    $height = $maxSize;
                }
            }
            //原图
            if ($size['ori'] == 1) {
                $sizeList[] = [
                    'min'   => $oriWidth,
                    'max'   => $oriHeight,
                    'price' => $size['price'] ?? 0,
                    'ori'   => $size['ori'] ?? 0,
                ];
            } elseif ($width > 0 && $height > 0) {
                $sizeList[] = [
                    'min'   => $width,
                    'max'   => $height,
                    'price' => $size['price'] ?? 0,
                    'ori'   => $size['ori'] ?? 0,
                ];
            }
        }
        unset($size);
    }
    $sizeList = myMultisort($sizeList, 'ori', SORT_ASC);

    return $sizeList;
}

/**
 * wcz
 * 获取活动类型
 *
 * @param  int  $type
 *
 * @return bool|mixed
 */
function getActivityType(int $type)
{
    $typeArr = [
        1 => '体验游',
        2 => '征集',
        3 => '直播',
        4 => '历史',
    ];

    return $typeArr[$type] ?? '';
}

/**
 * wcz
 * 获取Plog来源
 *
 * @param  int  $type
 *
 * @return bool|mixed
 */
function getSourceName(int $type)
{
    //来源(1:体验游,2:征集获奖图集,3:自购,4:直播活动,5:中台上传)
    $typeArr = [
        0 => '用户日常Plog',
        1 => '体验游活动Plog',
        2 => '征集活动Plog',
        3 => '自购Plog',
        4 => '直播活动Plog',
        5 => '自传Plog',
    ];

    return $typeArr[$type] ?? '';
}

/**
 * wcz
 * 通过身份证号获取年龄
 *
 * @param $idcard
 *
 * @return bool|false|string
 */
function getAgeByIdcard($idcard)
{
    if (!checkIdcard($idcard)) {
        return 0;
    }
    $year = substr($idcard, 6, 4);
    $monthDay = substr($idcard, 10, 4);
    $age = date('Y') - $year;
    if ($monthDay > date('md')) {
        $age--;
    }

    return $age;
}

/**
 * wcz
 * 获取活动状态
 *
 * @param  int  $status
 *
 * @return bool|mixed
 */
function getActivityStatus(int $status)
{
    $statusArr = [
        1 => '待发布',
        2 => '报名中',
        3 => '进行中',
        4 => '已结束',
        5 => '已关闭',
        6 => '等待报名',//创建成功到报名开始
        7 => '等待开始',//报名结束到活动开始
    ];

    return $statusArr[$status] ?? '';
}

/**
 * wcz
 * 获取帐户类型, 帐号类型(1:管理员,2:业主,3:摄影师,4:小编,5:普通用户)
 *
 * @param  int  $type
 *
 * @return bool|mixed
 */
function getAccountType(int $type)
{
    $typeArr = [
        1 => '管理员',
        2 => '业主',
        3 => '摄影师',
        4 => '小编',
        5 => '普通用户',
    ];

    return $typeArr[$type] ?? '';
}

/**
 * wcz
 * 获取分享方式, (1:默认微信,2:朋友圈,3:微博,4:分享链接,5:后台虚拟)
 *
 * @param  int  $way
 *
 * @return bool|mixed
 */
function getShareWay(int $way)
{
    $wayArr = [
        1 => '微信',
        2 => '朋友圈',
        3 => '微博',
        4 => '链接',
        5 => '后台虚拟',
    ];

    return $wayArr[$way] ?? '';
}

/**
 * wcz
 * 获取又拍云的图片验证参数
 *
 * @param $path
 * @param $type string 图片标志 a或者buy
 * @param  int  $external  是否对外图片, 1是0否
 *
 * @return string
 */
function getUpyunSign($path, $type, $external = 0)
{
    if ($external == 1) {
        $tokenTime = time() + config('picture.externalCoverTokenTime');
    } else {
        if ($type == 'a') {
            $tokenTime = time() + config('picture.coverTokenTime');
        } else {
            $tokenTime = time() + config('picture.tokenTime');
        }
    }
    $token = config('upyun.upyuntoken');
    $string = md5($token.'&'.$tokenTime.'&'.$path);
    $sign = substr($string, 12, 8).$tokenTime;

    return $sign;
}

/**
 * wcz
 * 获取设置防盗链的图片
 *
 * @param $url string 图片原图路径
 * @param  int  $picSize  float shortest=最短边大小, longest=最长边大小
 * @param $ori int 是否原图加密, 1原图, 0缩略图
 * @param $mark string 间隔标识符, 暂为!
 * @param $type string 缩略图版本, a为缩略图(有水印), buy为购买或举办活动拥有后的高清下载图片无水印
 *
 * @return string http://xxx.jpg?_upt=xxx
 */
function getTokenPicture($url, $picSize = 0, int $ori = 0, $mark = '!', $type = 'a')
{
    $url = getUrlByTokenUrl($url);
    if (empty($url)) {
        return $url;
    }
    $ori = empty($ori) ? 0 : $ori;
    //$mark = empty($mark) ? '!' : $mark;
    $mark = '!';
    $type = empty($type) ? 'a' : $type;
    if ($type == 'buy') {
        $coverSizeArr = $tmpSizeArr = config('picture.buySize') ?? [800, 1024, 1920];
    } else {
        $coverSizeArr = $tmpSizeArr = config('picture.coverSize') ?? [400, 500, 600, 800];
    }
    asort($tmpSizeArr);
    $minSize = $tmpSizeArr[0];
    $maxSize = end($tmpSizeArr);
    if ($maxSize < $picSize && $type == 'buy') {
        $ori = 1;
    }
    if (array_search($picSize, $coverSizeArr) === false && $ori != 1) {
        $picSize = $minSize;
    }
    //获取部分相对路径
    preg_match_all("/http(?:s?):[\/]{2}[a-z\d-]+(?:[.]{1}[a-z\d-]+)+/i", $url, $matches);
    $urlTmp = $matches[0] ?? [];
    if (empty($urlTmp[0]) || !in_array($type, ['a', 'buy'])) {
        return $url;
    }
    //图片部分路径path
    $path = str_replace($urlTmp[0], '', $url);
    if ($ori != 1) {
        $path = sprintf('%s%s%s%s', $path, $mark, $type, $picSize);
        $url = sprintf('%s%s%s%s', $url, $mark, $type, $picSize);
    }
    $upt = getUpyunSign($path, $type);
    $url = $url.'?_upt='.$upt;

    return $url;
}

/**
 * wcz
 * 获取设置防盗链的视频
 *
 * @param $url string 路径
 *
 * @return string http://xxx.mp4?_upt=xxx
 */
function getTokenVideoUrl($url)
{
    $url = getUrlByTokenUrl($url);
    if (empty($url)) {
        return $url;
    }
    //获取部分相对路径
    preg_match_all("/http(?:s?):[\/]{2}[a-z\d-]+(?:[.]{1}[a-z\d-]+)+/i", $url, $matches);
    $urlTmp = $matches[0] ?? [];
    if (empty($urlTmp[0])) {
        return $url;
    }
    //图片部分路径path
    $path = str_replace($urlTmp[0], '', $url);
    $upt = getUpyunSign($path, 'a');
    $url = $url.'?_upt='.$upt;

    return $url;
}

/**
 * wcz
 * 获取设置防盗链的图片,第三方
 *
 * @param $url string 图片原图路径
 * @param  int  $picSize  float shortest=最短边大小, longest=最长边大小
 * @param $ori int 是否原图加密, 1原图, 0缩略图
 * @param $mark string 间隔标识符, 暂为!
 * @param $type string 缩略图版本, a为缩略图(有水印), buy为购买或举办活动拥有后的高清下载图片无水印
 *
 * @return string http://xxx.jpg?_upt=xxx
 */
function getExternalTokenPicture($url, $picSize = 0, int $ori = 0, $mark = '!', $type = 'a')
{
    $url = getUrlByTokenUrl($url);
    if (empty($url)) {
        return $url;
    }
    $ori = empty($ori) ? 0 : $ori;
    $mark = empty($mark) ? '!' : $mark;
    $type = empty($type) ? 'a' : $type;
    if ($type == 'buy') {
        $coverSizeArr = $tmpSizeArr = config('picture.buySize') ?? [1024, 1920];
    } else {
        $coverSizeArr = $tmpSizeArr = config('picture.coverSize') ?? [400, 500, 600, 800];
    }
    asort($tmpSizeArr);
    $minSize = $tmpSizeArr[0];
    $maxSize = end($tmpSizeArr);
    if ($maxSize < $picSize && $type == 'buy') {
        $ori = 1;
    }
    if (array_search($picSize, $coverSizeArr) === false && $ori != 1) {
        $picSize = $minSize;
    }
    //获取部分相对路径
    preg_match_all("/http(?:s?):[\/]{2}[a-z\d-]+(?:[.]{1}[a-z\d-]+)+/i", $url, $matches);
    $urlTmp = $matches[0] ?? [];
    if (empty($urlTmp[0]) || !in_array($type, ['a', 'buy'])) {
        return $url;
    }
    //图片部分路径path
    $path = str_replace($urlTmp[0], '', $url);
    if ($ori != 1) {
        $path = sprintf('%s%s%s%s', $path, $mark, $type, $picSize);
        $url = sprintf('%s%s%s%s', $url, $mark, $type, $picSize);
    }
    $upt = getUpyunSign($path, $type, 1);
    $url = $url.'?_upt='.$upt;

    return $url;
}

/**
 * 百度API, 智能获取图片标签
 *
 * @param $token
 * @param $img
 * @param  int  $num
 *
 * @return array
 */
function getImageClassify($token, $img, $num = 2)
{
    $arr = [];
    $image = $img ?? '';
    if ($image && $token) {
        try {
            $url = 'https://aip.baidubce.com/rest/2.0/image-classify/v2/advanced_general?access_token='.$token;
            if ( !preg_match("/data:(.*?);base64,(.*?)$/si", $image)) {
                $image = file_get_contents($image);
                $image = base64_encode($image);
                $body = [
                    'image' => $image,
                ];
            } else {
                $body = [
                    'url' => $image,
                ];
            }
            $return = curlHttp($url, $body, 'POST');
            if ( !empty($return) && $return['code'] == 200) {
                $response = json_decode($return['response'], true);
                if ( !empty($response['result'])) {
                    $numTmp = 0;
                    foreach ($response['result'] as &$val) {
                        if ($val['score'] >= 0.3 && $val['keyword']) {
                            $arr[] = htmlentities(trim($val['keyword']));
                            $numTmp++;
                        }
                        if ($num && $numTmp == $num) {
                            break;
                        }
                    }
                    unset($val);
                }
            }
        } catch (Exception $e) {
            $msg = $e->getMessage();
            $line = $e->getFile().':'.$e->getLine();
            WriteLog::error(sprintf('智能获取图片[%s]标签失败, %s %s', $line, $msg, $img));
        }
    }

    return $arr;
}

/**
 * 处理并返回标签
 *
 * @param $tagsTmp
 * @param  string  $type  tags类型:json为json数组, 默认空值为普通字符串
 *
 * @return array
 */
function getTags($tagsTmp, $type = ',')
{
    if ( !$tagsTmp) {
        return [];
    }
    $tags = [];
    if ( !is_array($tagsTmp)) {
        if ($type == 'json') {
            $tags = json_decode($tagsTmp, true);
        } else {
            $tags = getTagName($tagsTmp, $type);
            $tags = explode($type, $tags);
        }
        $tags = (array) $tags;
    }
    if (empty($tags)) {
        return $tags;
    }
    $tags = array_unique(array_filter($tags), SORT_REGULAR);
    sort($tags);

    return $tags;
}

/**
 * 单词过滤
 *
 * @param $word
 * @param  string  $type
 *
 * @return string|string[]|null
 */
function getTagName($word, $type = ',')
{
    if ( !empty($word)) {
        // Filter 英文标点符号 |  Filter 连续空格
        $word = preg_replace("/[[:punct:]|\s+]/i", $type, $word);
        // Filter 中文标点符号
        mb_regex_encoding('utf-8');
        //转换，、
        $word = mb_ereg_replace("[，、]", ',', $word, "UTF-8");
        $char = "。！？：；﹑•＂…‘’“”〝〞∕¦‖—　〈〉﹞﹝「」‹›〖〗】【»«』『〕〔》《﹐¸﹕︰﹔！¡？¿﹖﹌﹏﹋＇´ˊˋ―﹫︳︴¯＿￣﹢﹦﹤‐­˜﹟﹩﹠﹪﹡﹨﹍﹉﹎﹊ˇ︵︶︷︸︹︿﹀︺︽︾ˉ﹁﹂﹃﹄︻︼（）";
        $word = mb_ereg_replace("[".$char."]", '', $word, "UTF-8");
        if ($word) {
            $wordArr = [];
            $wordArrTmp = explode($type, $word);
            if ($wordArrTmp) {
                foreach ($wordArrTmp as &$item) {
                    preg_match('/[\x{4e00}-\x{9fff}\d\w\s]+/u', $item, $result);
                    if ( !empty($result[0])) {
                        $wordArr[] = $result[0];
                    }
                }
                unset($item);
            }
            $word = $wordArr ? implode($type, array_unique(array_filter($wordArr))) : '';
        }
    }

    return $word;
}

/*wg
 * 时长格式转换
 * param $second 秒(只支持一年以内的)
 * return *天*时*分$秒
 */
function durationFormat($seconds)
{

    if (empty($seconds)) {
        return '0秒';
    }
    if ( !is_int($seconds) || $seconds <= 0 || $seconds >= (365 * 86400)) {
        return false;
    }
    $duration = '';
    if ($seconds <= 0) {
        return $duration;
    }
    [$day, $hour, $minute, $second] = explode(' ', gmstrftime('%j %H %M %S', $seconds));
    $day -= 1;
    if ($day > 0) {
        $duration .= (int) $day.'天';
    }
    if ($hour > 0) {
        $duration .= (int) $hour.'小时';
    }
    if ($minute > 0) {
        $duration .= (int) $minute.'分钟';
    }
    if ($second > 0) {
        $duration .= (int) $second.'秒';
    }

    return $duration;
}

/**
 * 获取今日开始时间戳
 *
 * @return int
 */
function getBeginTodayTimestamp()
{
    $time = mktime(0, 0, 0, date('m'), date('d'), date('Y'));

    return (int) $time;
}

/**
 * 获取今日结束时间戳
 *
 * @return int
 */
function getEndTodayTimestamp()
{
    $time = mktime(0, 0, 0, date('m'), date('d') + 1, date('Y')) - 1;

    return (int) $time;
}

/**
 * wcz
 * 获取身份证上的性别
 *
 * @param $idcard
 *
 * @return string|null
 */
function getGenderByIdcard($idcard)
{
    if (!checkIdcard($idcard)) {
        return 0;
    }
    $genderInt = (int) substr($idcard, 16, 1);

    return $genderInt % 2 === 0 ? 2 : 1; //2女 1男
}

//身份证验证
function checkIdcard($idcard)
{
    if ($idcard) {
        if (preg_match('/^(\d{6}(18|19|20)?\d{2}(0[1-9]|1[012])(0[1-9]|[12]\d|3[01])\d{3}(\d|[xX]))$/', $idcard)) {
            return true;
        }
    }

    return false;
}

/**
 * 护照
 * 1.G|E+8位数字：如：G12345678
 * 2.D|S|P+7位数字：如：D1234567
 */
function passportVerify($content)
{

    // 护照
    // 规则： 14/15开头 + 7位数字, G + 8位数字, P + 7位数字, S/D + 7或8位数字,等
    // 样本： 141234567, G12345678, P1234567
    $string = "/^[14|15][0-9]\d{7}$|^[P|p|S|s]\d{7}$|^[S|s|G|g]\d{8}$|^[E|e|G|g]\d{8}$|^[Gg|Tt|Ss|Ll|Qq|Dd|Aa|Ff]\d{8}$|^[H|h|M|m]\d{8,10}$/";
    if (preg_match($string, $content)) {
        return true;
    }

    return false;
}

/**
 * 根据地址字符串获取省市区域
 *
 * @param $location
 *
 * @return array
 */
function getAddressDetail($location)
{
    $province = $city = $area = '';
    preg_match('/(.*?(省|自治区))/isu', $location, $matches);
    if (count($matches) > 1) {
        $province = $matches[count($matches) - 2];
        $location = str_replace($province, '', $location);
    }
    preg_match('/(.*?(市|自治州|地区|区划|县))/isu', $location, $matches);
    if (count($matches) > 1) {
        $city = $matches[count($matches) - 2];
        $location = str_replace($city, '', $location);
    } else {
        if ($province == '北京市' || $province == '天津市' || $province == '上海市' || $province == '重庆市') {
            $city = '市辖区';
        }
    }
    preg_match('/(.*?((?<!景)区))/isu', $location, $matches);
    if (count($matches) > 1) {
        $area = $matches[count($matches) - 2];
        $location = str_replace($area, '', $location);
    }
    preg_match('/(.*?(县|镇|乡|街道))/isu', $location, $matches);
    if (count($matches) > 1) {
        $addr = $matches[count($matches) - 2];
        $location = str_replace($addr, '', $location);
    }
    //$province = $city = $area = '';
    //preg_match('/(.*?省)/', $location, $matches);
    //if (count($matches) > 1) {
    //    $province = $matches[count($matches) - 2];
    //    $location = str_replace($province, '', $location);
    //}
    //preg_match_all('/(.*?(市|自治州|地区|区划|县))/', $location, $matches);
    //if (count($matches) > 1) {
    //    $city = $matches[count($matches) - 2];
    //    $location = str_replace($city, '', $location);
    //}else {
    //    if ($province == '北京市' || $province == '天津市' || $province == '上海市' || $province == '重庆市') {
    //        $city = '市辖区';
    //    }
    //}
    //preg_match('/(.*?((?<!景)区|县|镇|乡|街道))/', $location, $matches);
    //if (count($matches) > 1) {
    //    $area = $matches[count($matches) - 2];
    //    $location = str_replace($area, '', $location);
    //}
    return [
        'province' => $province ?? '',
        'city'     => $city ?? '',
        'area'     => $area ?? '',
        'addr'     => $addr ?? '',
        'address'  => $location ?? '',
    ];
}

/**
 * 网络图转base64编码
 *
 * @param  string img 图片网址
 **@return bool|string
 */
function imgToBase64($img = '')
{
    if ( !$img) {
        return false;
    }
    try {
        $url = getUrlByTokenUrl($img);
        $extension = substr(strrchr($url, '.'), 1);
        $extension = $extension ? strtolower($extension) : '';
        $mine = '';
        if (in_array($extension, ['jpg', 'jpeg'])) {
            $mine = 'image/jpeg';
        } elseif ($extension == 'png') {
            $mine = 'image/png';
        } elseif ($extension == 'gif') {
            $mine = 'image/gif';
        }
        if (empty($mine)) {
            $imageInfo = getimagesize($img);
            $mine = $imageInfo['mine'] ?? 'image/jpeg';
        }
        $base64 = chunk_split(base64_encode(file_get_contents($img)));

        return 'data:'.$mine.';base64,'.$base64;
    } catch (Exception $e) {
        $msg = $e->getMessage();
        $line = $e->getFile().':'.$e->getLine();
        WriteLog::error(sprintf('图片[%s]转码失败, %s %s', $img, $line, $msg));

        return $img;
    }
}

/**
 * 标签ID随机取模,输出表名
 *
 * @param $number
 * @param  int  $mold
 * @param  string  $table
 *
 * @return string
 */
function getTagsTable($number, $mold = 8, $table = 'tags')
{
    if (is_numeric($number) && is_numeric($mold)) {
        return $table.'_'.($number % $mold);
    }

    return '';
}

/**
 * 标签ID随机取模,输出表名
 *
 * @param $number
 * @param  int  $mold
 * @param  string  $table
 *
 * @return string
 */
function getTagsOrPoiRelevanceTable($number, $baseTable = '', $mold = 8)
{
    if (is_numeric($number) && is_numeric($mold)) {
        return $baseTable.'_'.($number % $mold);
    }

    return '';
}

/**
 * 生成标签ID
 *
 * @param $userId
 * @param  int  $randCount
 *
 * @return string
 */
function createTagsId($userId, $randCount = 8)
{
    $userId = str_pad($userId, 8, 0, STR_PAD_RIGHT);
    $randNum = randomCode($randCount, 'num');

    return $userId.$randNum;
}

/**
 * 处理并返回数组
 *
 * @param $tags string|array
 * @param  string  $type  tags类型:json为json数组, 默认空值为普通字符串
 *
 * @return array
 */
function getStringToArray($tags, $type = ',')
{
    if ( !$tags) {
        return [];
    }
    if ( !is_array($tags)) {
        if ($type == 'json') {
            $tags = json_decode($tags, true);
        } else {
            $tags = explode($type, $tags);
        }
    }
    $tags = (array) $tags;

    return array_unique(array_filter($tags), SORT_REGULAR);
}

/**
 *计算某个经纬度的周围某段距离的正方形的四个点
 *
 * @param $lng float 经度
 * @param $lat float 纬度
 * @param  float  $distance  该点所在圆的半径，该圆与此正方形内切，默认值为10千米
 *
 * @return array 正方形的四个点的经纬度坐标
 */
function returnSquarePoint($lng, $lat, $distance = 10.0)
{
    $EARTH_RADIUS = 6371; //地球半径，平均半径为6371km
    $dlng = 2 * asin(sin($distance / (2 * $EARTH_RADIUS)) / cos(deg2rad($lat)));
    $dlng = rad2deg($dlng);
    $dlat = $distance / $EARTH_RADIUS;
    $dlat = rad2deg($dlat);

    return [
        'left-top'     => [
            'lat' => $lat + $dlat,
            'lng' => $lng - $dlng,
        ],
        'right-top'    => [
            'lat' => $lat + $dlat,
            'lng' => $lng + $dlng,
        ],
        'left-bottom'  => [
            'lat' => $lat - $dlat,
            'lng' => $lng - $dlng,
        ],
        'right-bottom' => [
            'lat' => $lat - $dlat,
            'lng' => $lng + $dlng,
        ],
    ];
}

/**
 * 针对某一个键值对二维数组进行去重
 *
 * @param $arr
 * @param $key
 *
 * @return mixed
 */
function secondArrayUnique($arr, $key)
{
    $tmpArr = [];
    foreach ($arr as $k => $v) {
        if (in_array($v[$key], $tmpArr)) {
            unset($arr[$k]);
        } else {
            $tmpArr[$k] = $v[$key];
        }
    }
    $arr = array_values($arr);

    return $arr;
}

/**
 * 通过营业执照图片获取公司信息
 *
 * @param $image  :图片地址
 *
 */
function getBusinessLicenseInfo($image)
{
    try {
        $image_url = empty($image) ? '' : $image;
        //API产品路径
        $host = "http://blicence.market.alicloudapi.com";
        $path = "/ai_business_license";
        $method = "POST";
        //阿里云APPCODE
        $appcode = "63a2e8903b0b4ad391d00dbc78679117";
        $headers = [];
        array_push($headers, "Authorization:APPCODE ".$appcode);
        array_push($headers, "Content-Type".":"."application/x-www-form-urlencoded; charset=UTF-8");
        #启用URL方式进行识别
        #内容数据类型是图像文件URL链接
        $AI_BUSINESS_LICENSE_IMAGE = $image_url;
        $AI_BUSINESS_LICENSE_IMAGE = urlencode($AI_BUSINESS_LICENSE_IMAGE);
        $AI_BUSINESS_LICENSE_IMAGE_TYPE = "1";
        $bodys = "AI_BUSINESS_LICENSE_IMAGE=".$AI_BUSINESS_LICENSE_IMAGE."&AI_BUSINESS_LICENSE_IMAGE_TYPE=".$AI_BUSINESS_LICENSE_IMAGE_TYPE;
        $url = $host.$path;
        $curl = curl_init();
        curl_setopt($curl, CURLOPT_CUSTOMREQUEST, $method);
        curl_setopt($curl, CURLOPT_URL, $url);
        curl_setopt($curl, CURLOPT_TIMEOUT, 3);  //只需要设置一个秒的数量就可以
        curl_setopt($curl, CURLOPT_HTTPHEADER, $headers);
        curl_setopt($curl, CURLOPT_FAILONERROR, false);
        curl_setopt($curl, CURLOPT_RETURNTRANSFER, true);
        //curl_setopt($curl, CURLOPT_HEADER, true);
        if (1 == strpos("$".$host, "https://")) {
            curl_setopt($curl, CURLOPT_SSL_VERIFYPEER, false);
            curl_setopt($curl, CURLOPT_SSL_VERIFYHOST, false);
        }
        curl_setopt($curl, CURLOPT_POSTFIELDS, $bodys);
        $T = curl_exec($curl);
        $businessData = [];
        $businessLicense = [];
        $businesstype = [];
        $returnData = [];
        $endData = [];
        if ( !empty($T)) {
            $T = ltrim($T, '{');
            $T = rtrim($T, '}');
            $businessData = explode('",', $T);
            //商户类型（1:民宿2：景区 3：特产 4：导游服务 5：餐饮）
            $businesstype = config('businessLicense');
            foreach ($businessData as $key => $value) {
                $arr = explode(':', $value);
                $arr[0] = trim(str_replace(['"', '"', '"'], "", $arr[0]));
                $arr[1] = trim(str_replace(['"', '"', '"'], "", $arr[1]));
                $returnData[$arr[0]] = $arr[1];
            }
            foreach ($returnData as $key => $value) {
                if ( !empty($businesstype[$key])) {
                    $businessLicense[$businesstype[$key]] = $value;
                }
            }
        }

        return $businessLicense;
    } catch (\Exception $e) {
        return [];
    }

}

/**
 * 指定概率随机数
 *
 * @param  array  $randArray
 *
 * @return mixed
 */
function roll($randArray = [])
{
    $roll = rand(1, array_sum($randArray));
    $_tmpW = 0;
    $rollNum = 1;
    foreach ($randArray as $k => $v) {
        $min = $_tmpW;
        $_tmpW += $v;
        $max = $_tmpW;
        if ($roll > $min && $roll <= $max) {
            $rollNum = $k;
            break;
        }
    }

    return $rollNum;
}

/**
 * 加密,解密ID
 *
 * @param $string
 * @param  string  $action
 * @param  int  $length
 *
 * @return false|string
 */
function authCodeId($string, $action = 'encode', $length = 16)
{
    $startLen = 7;
    $endLen = $length - $startLen > 7 ? $length - $startLen : 7;
    $code = '';
    if ($action == 'encode') {
        $idLen = strlen($string);
        $salt = config('app.client_app_key');
        $codeDtr = $string.$salt;
        $enCodeStr = hash('md5', $codeDtr);
        $code = $idLen.substr($enCodeStr, 8, $startLen - $idLen).$string.substr($enCodeStr, -8, $endLen);
        $code = strtoupper($code);
    } elseif ($action == 'decode') {
        $idLen = $string[0];
        $code = substr($string, $startLen - $idLen + 1, $idLen);
    }

    return $code;
}

/**
 *获取最近n天的日期
 *
 * @param  string  $time
 * @param  string  $format
 * @param $num
 *
 * @return array
 */
function getPeriodDate($time = '', $format = 'Y-m-d', $num = 7)
{
    $time = $time != '' ? $time : time();
    $date = [];
    for ($i = 0; $i < $num; $i++) {
        $date[$i] = date($format, strtotime('+'.($i - $num).' days', $time));
    }

    return $date;
}

/**
 * wcz
 * 获取活动类型
 *
 * @param  int  $type
 *
 * @return bool|mixed
 */
function getActivityTable(int $type)
{
    $typeArr = [
        1 => 'activityTour',
        2 => 'activityLevy',
        3 => 'activityLive',
        4 => 'activityHistory',
    ];

    return $typeArr[$type] ?? '';
}

/**
 * has
 * 获取摄影师头衔
 *
 * @param    $ids
 *
 * @return array
 */
function getRankTitleName($ids)
{
    $returnData = [];
    $allId = $ids;
    if ( !is_array($allId)) {
        $allId = explode(',', $allId);
    }
    $rankTitle = config('rankTitle');
    if ( !empty($rankTitle)) {
        foreach ($rankTitle as $key => $value) {
            if (in_array($value['id'], $allId)) {
                $returnData[] = $value['name'];
            }
        }
    }

    return $returnData;
}

//是否过期
function isExpired($expiredTime)
{
    $expiredEnabled = config('picture.expiredEnabled') ?? 0;

    return $expiredEnabled && $expiredTime && $expiredTime <= time() ? 1 : 0;
}

/**
 * @param $timestamp
 * @param  int  $datemac
 *
 * @return false|string
 */
function getDateFormat($timestamp, $datemac = 0)
{
    if (empty($timestamp)) {
        $timestamp = time();
    }
    $y = date("Y");
    $m = date("m");
    $d = date("d");
    $dateformat = "Y-m-d H:i:s";
    if (time() >= $timestamp) {
        $time = abs(time() - $timestamp);
        if ($time < 10) {
            return "刚刚";
        } elseif ($time < 60) {
            return "{$time}秒前";
        } elseif ($time < 30 * 60) {
            $i = intval($time / 60) > 0 ? intval($time / 60) : 1;
            $s = ceil(abs($time - $i * 60));
            if ($s > 0) {
                return "{$i}分{$s}秒前";
            } else {
                return "{$i}分钟前";
            }
        } elseif ($timestamp > mktime(0, 0, 0, $m, $d, $y) && $timestamp < mktime(0, 0, 0, $m, $d + 1, $y)) {
            $dateformat = "H:i";
        } elseif ($timestamp > mktime(0, 0, 0, $m, $d - 1, $y) && $timestamp < mktime(0, 0, 0, $m, $d, $y)) {
            $dateformat = "昨天H:i";
        } elseif ($timestamp > mktime(0, 0, 0, $m, $d - 2, $y) && $timestamp < mktime(0, 0, 0, $m, $d - 1, $y)) {
            $dateformat = "前天H:i";
        } elseif ($timestamp > mktime(0, 0, 0, $m, 1, $y) && $timestamp < mktime(0, 0, 0, $m + 1, 1, $y)) {
            $dateformat = "当月d日 H:i";
        } elseif ($timestamp > mktime(0, 0, 0, 1, 1, $y) && $timestamp < mktime(0, 0, 0, 1, 1, $y + 1)) {
            $dateformat = "m月d日 H:i";
        } elseif ($timestamp > mktime(0, 0, 0, 1, 1, $y - 1) && $timestamp < mktime(0, 0, 0, 1, 1, $y)) {
            $dateformat = "去年m月d日 H:i";
        } elseif ($timestamp > mktime(0, 0, 0, 1, 1, $y - 2) && $timestamp < mktime(0, 0, 0, 1, 1, $y - 1)) {
            $dateformat = "前年m月d日 H:i";
        }
    }
    if (function_exists("gmdate")) {
        return gmdate($dateformat, $timestamp + $datemac);
    } else {
        return date($dateformat, $timestamp + $datemac);
    }
}

/**
 * 高德API, 逆地理编码：通过经纬度获取地址详细信息
 *
 * @param $params
 *
 * @return array
 */
function getGaodeLocationInfo($params)
{
    $key = config('gaode.key');
    $arr = [];
    if ($key) {
        $params['key'] = $key;
        $params['location'] = $params['location'] ?? '';//经纬度逗号分割
        $params['radius'] = $params['radius'] ?? 0;//搜索半径,radius取值范围在0~3000，默认是1000。单位：米
        $params['extensions'] = $params['extensions'] ?? 'all';//extensions 参数默认取值是 base，也就是返回基本地址信息；extensions 参数取值为 all 时会返回基本地址信息、附近 POI 内容、道路信息以及道路交叉口信息。
        $params['batch'] = $params['batch'] ?? false;//batch 参数设置为 true 时进行批量查询操作，最多支持 20 个经纬度点进行批量地址查询操作。batch 参数设置为 false 时进行单点查询，此时即使传入多个经纬度也只返回第一个经纬度的地址解析查询结果。
        $params['poitype'] = $params['poitype'] ?? '';
        $params['roadlevel'] = $params['roadlevel'] ?? 1;//可选值：1，当roadlevel=1时，过滤非主干道路，仅输出主干道路数据
        try {
            $url = 'https://restapi.amap.com/v3/geocode/regeo';
            $url = $url.'?'.http_build_query($params);
            $return = curlHttp($url, '', '');
            if ( !empty($return) && $return['code'] == 200) {
                $response = json_decode($return['response'] ?? [], true);
                $arr = $response['regeocode'] ?? [];
            }
        } catch (Exception $e) {
            $msg = $e->getMessage();
            $line = $e->getFile().':'.$e->getLine();
            WriteLog::error(sprintf('获取地理编码失败, %s %s |data:%s', $line, $msg, json_encode($params, JSON_UNESCAPED_UNICODE)));
        }
    }

    return $arr;
}

/**
 * 高德API, 周边搜索：通过经纬度获取周边地址详细信息
 *
 * @param $params
 *
 * @return array
 */
function getGaodePlaceAroundInfo($params)
{
    $key = config('gaode.key');
    $arr = [];
    if ($key) {
        $params['key'] = $key;
        $params['location'] = $params['location'] ?? '';//经纬度逗号分割
        $params['keywords'] = $params['keywords'] ?? '';
        $params['radius'] = $params['radius'] ?? 1000;//搜索半径,radius取值范围在0~3000，默认是1000。单位：米
        $params['extensions'] = $params['extensions'] ?? 'all';//extensions 参数默认取值是 base，也就是返回基本地址信息；extensions 参数取值为 all 时会返回基本地址信息、附近 POI 内容、道路信息以及道路交叉口信息。
        $params['offset'] = $params['offset'] ?? '1';//每页记录数据
        $params['types'] = $params['types'] ?? '';//查询POI类型
        $params['sortrule'] = $params['sortrule'] ?? 'distance';//规定返回结果的排序规则。按距离排序：distance；综合排序：weight
        try {
            $url = 'https://restapi.amap.com/v3/place/around';
            $url = $url.'?'.http_build_query($params);
            $return = curlHttp($url, '', '');
            if ( !empty($return) && $return['code'] == 200) {
                $response = json_decode($return['response'] ?? [], true);
                $arr = $response['pois'] ?? [];
            }
        } catch (Exception $e) {
            $msg = $e->getMessage();
            $line = $e->getFile().':'.$e->getLine();
            WriteLog::error(sprintf('获取地理编码失败, %s %s |data:%s', $line, $msg, json_encode($params, JSON_UNESCAPED_UNICODE)));
        }
    }

    return $arr;
}

/**
 * 高德API, 地理编码: 通过地址,城市信息获取具体信息
 *
 * @param $params
 *
 * @return array
 */
function getGaodeCodes($params)
{
    $key = config('gaode.key');
    $arr = [];
    if ($key) {
        $params['key'] = $key;
        $params['address'] = $params['address'] ?? '';//结构化地址信息:省份＋城市＋区县＋城镇＋乡村＋街道＋门牌号码
        $params['city'] = $params['city'] ?? '';//查询城市，可选：城市中文、中文全拼、citycode、adcode
        try {
            $url = 'https://restapi.amap.com/v3/geocode/geo';
            $url = $url.'?'.http_build_query($params);
            $return = curlHttp($url, '', '');
            if ( !empty($return) && $return['code'] == 200) {
                $response = json_decode($return['response'] ?? [], true);
                $arr = $response['geocodes'] ?? [];
            }
        } catch (Exception $e) {
            $msg = $e->getMessage();
            $line = $e->getFile().':'.$e->getLine();
            WriteLog::error(sprintf('获取地理编码失败, %s %s |data:%s', $line, $msg, json_encode($params, JSON_UNESCAPED_UNICODE)));
        }
    }

    return $arr;
}

/**
 * 根据GPS信息获取经纬度
 *
 * @param $gpsLongitude
 * @param $gpsLatitude
 */
function getGpsToCoordinate($gpsLongitude, $gpsLatitude)
{
    $lng = 0;
    $lat = 0;
    if ($gpsLongitude && $gpsLatitude) {
        $lngArr = explode(',', $gpsLongitude);
        $latArr = explode(',', $gpsLatitude);
        foreach ($lngArr as $k => $v) {
            $arr = explode('/', $v);
            $num = empty($arr[0]) || empty($arr[1]) ? 0 : $arr[0] / $arr[1];
            if ($k == 1) {
                $num = $num / 60;
            } elseif ($k == 2) {
                $num = $num / 3600;
            }
            $lng += $num;
        }
        foreach ($latArr as $k => $v) {
            $arr = explode('/', $v);
            $num = empty($arr[0]) || empty($arr[1]) ? 0 : $arr[0] / $arr[1];
            if ($k == 1) {
                $num = $num / 60;
            } elseif ($k == 2) {
                $num = $num / 3600;
            }
            $lat += $num;
        }
    }

    return [
        'lng' => empty($lng) ? '' : $lng,
        'lat' => empty($lat) ? '' : $lat,
    ];
}

/**
 * 根据时间戳获取节气
 *
 * @param  int  $dateline
 *
 * @return array
 */
function getJieQiByDate($dateline = 0)
{
    $dateline = empty($dateline) ? time() : $dateline;
    $_year = date('Y', $dateline);
    $month = date('m', $dateline);
    $day = date('d', $dateline);
    $year = (int) substr($_year, -2) + 0;
    $coefficient = [
        [5.4055, 2019, -1],//小寒
        [20.12, 2082, 1],//大寒
        [3.87],//立春
        [18.74, 2026, -1],//雨水
        [5.63],//惊蛰
        [20.646, 2084, 1],//春分
        [4.81],//清明
        [20.1],//谷雨
        [5.52, 1911, 1],//立夏
        [21.04, 2008, 1],//小满
        [5.678, 1902, 1],//芒种
        [21.37, 1928, 1],//夏至
        [7.108, 2016, 1],//小暑
        [22.83, 1922, 1],//大暑
        [7.5, 2002, 1],//立秋
        [23.13],//处暑
        [7.646, 1927, 1],//白露
        [23.042, 1942, 1],//秋分
        [8.318],//寒露
        [23.438, 2089, 1],//霜降
        [7.438, 2089, 1],//立冬
        [22.36, 1978, 1],//小雪
        [7.18, 1954, 1],//大雪
        [21.94, 2021, -1]//冬至
    ];
    $termName = [
        [
            'name'  => '小寒', 'pic' => '', 'note' => '小寒进入三九天，丰收致富庆元旦，冬季参加培训班，不断总结新经验。', 'name_en' => 'Minor Cold',//英文翻译来自于2022年冬奥会开幕式倒计时
            'color' => '#69c',
        ],
        [
            'name'  => '大寒', 'pic' => '', 'note' => '大寒虽冷农户欢，富民政策夸不完，联产承包继续干，欢欢喜喜过个年。', 'name_en' => 'Major Cold',
            'color' => '#69c',
        ],
        [
            'name'  => '立春', 'pic' => '', 'note' => '立春春打六九头，春播备耕早动手，一年之计在于春，农业生产创高优。', 'name_en' => 'Beginning of Spring',
            'color' => '#9C0',
        ],
        [
            'name'  => '雨水', 'pic' => '', 'note' => '雨水春雨贵如油，顶凌耙耘防墒流，多积肥料多打粮，精选良种夺丰收。', 'name_en' => 'Rain Water',
            'color' => '#9C0',
        ],
        [
            'name'  => '惊蛰', 'pic' => '', 'note' => '惊蛰天暖地气开，冬眠蛰虫苏醒来，冬麦镇压来保墒，耕地耙耘种春麦。', 'name_en' => 'Awakening of Insects',
            'color' => '#9C0',
        ],
        [
            'name'  => '春分', 'pic' => '', 'note' => '春分风多雨水少，土地解冻起春潮，稻田平整早翻晒，冬麦返青把水浇。', 'name_en' => 'Spring Equinox',
            'color' => '#9C0',
        ],
        [
            'name'  => '清明', 'pic' => '', 'note' => '清明春始草青青，种瓜点豆好时辰，植树造林种甜菜，水稻育秧选好种。', 'name_en' => 'Pure Brightness',
            'color' => '#9C0',
        ],
        [
            'name'  => '谷雨', 'pic' => '', 'note' => '谷雨雪断霜未断，杂粮播种莫迟延，家燕归来淌头水，苗圃枝接耕果园。', 'name_en' => 'Grain Rain',
            'color' => '#9C0',
        ],
        [
            'name'  => '立夏', 'pic' => '', 'note' => '立夏麦苗节节高，平田整地栽稻苗，中耕除草把墒保，温棚防风要管好。', 'name_en' => 'Beginning of Summer',
            'color' => '#390',
        ],
        [
            'name'  => '小满', 'pic' => '', 'note' => '小满温和春意浓，防治蚜虫麦秆蝇，稻田追肥促分孽，抓绒剪毛防冷风。', 'name_en' => 'Grain Buds',
            'color' => '#390',
        ],
        [
            'name'  => '芒种', 'pic' => '', 'note' => '芒种雨少气温高，玉米间苗和定苗，糜谷荞麦抢墒种，稻田中耕勤除草。', 'name_en' => 'Grain in Ear',
            'color' => '#390',
        ],
        [
            'name'  => '夏至', 'pic' => '', 'note' => '夏至夏始冰雹猛，拔杂去劣选好种，消雹增雨干热风，玉米追肥防粘虫。', 'name_en' => 'Summer Solstice',
            'color' => '#390',
        ],
        [
            'name'  => '小暑', 'pic' => '', 'note' => '小暑进入三伏天，龙口夺食抢时间，玉米中耕又培土，防雨防火莫等闲。', 'name_en' => 'Minor Heat',
            'color' => '#390',
        ],
        [
            'name'  => '大暑', 'pic' => '', 'note' => '大暑大热暴雨增，复种秋菜紧防洪，勤测预报稻瘟病，深水护秧防低温。', 'name_en' => 'Major Heat',
            'color' => '#390',
        ],
        [
            'name'  => '立秋', 'pic' => '', 'note' => '立秋秋始雨淋淋，及早防治玉米螟，深翻深耕土变金，苗圃芽接摘树心。', 'name_en' => 'Beginning of Autumn',
            'color' => '#f90',
        ],
        [
            'name'    => '处暑', 'pic' => '', 'note' => '处暑伏尽秋色美，玉主甜菜要灌水，粮菜后期勤管理，冬麦整地备种肥。',
            'name_en' => 'End of Heat',
            'color'   => '#f90',
        ],
        [
            'name'  => '白露', 'pic' => '', 'note' => '白露夜寒白天热，播种冬麦好时节，灌稻晒田收葵花，早熟苹果忙采摘。', 'name_en' => 'White Dew',
            'color' => '#f90',
        ],
        [
            'name'  => '秋分', 'pic' => '', 'note' => '秋分秋雨天渐凉，稻黄果香秋收忙，碾谷脱粒交公粮，山区防霜听气象。', 'name_en' => 'Autumn Equinox',
            'color' => '#f90',
        ],
        [
            'name'  => '寒露', 'pic' => '', 'note' => '寒露草枯雁南飞，洋芋甜菜忙收回，管好萝卜和白菜，秸秆还田秋施肥。', 'name_en' => 'Cold Dew',
            'color' => '#f90',
        ],
        [
            'name'  => '霜降', 'pic' => '', 'note' => '霜降结冰又结霜，抓紧秋翻蓄好墒，防冻日消灌冬水，脱粒晒谷修粮仓。', 'name_en' => "Frost's Descent",
            'color' => '#f90',
        ],
        [
            'name'  => '立冬', 'pic' => '', 'note' => '立冬地冻白天消，羊只牲畜圈修牢，培田整地修渠道，农田建设掀高潮。', 'name_en' => 'Beginning of Winter',
            'color' => '#69c',
        ],
        [
            'name'  => '小雪', 'pic' => '', 'note' => '小雪地封初雪飘，幼树葡萄快埋好，利用冬闲积肥料，庄稼没肥瞎胡闹。', 'name_en' => 'Minor Snow',
            'color' => '#69c',
        ],
        [
            'name'  => '大雪', 'pic' => '', 'note' => '大雪腊雪兆丰年，多种经营创高产，及时耙耘保好墒，多积肥料找肥源。', 'name_en' => 'Major Snow',
            'color' => '#69c',
        ],
        [
            'name'  => '冬至', 'pic' => '', 'note' => '冬至严寒数九天，羊只牲畜要防寒，积极参加夜技校，增产丰收靠科研。', 'name_en' => 'Winter Solstice',
            'color' => '#69c',
        ],
    ];
    $idx1 = ($month - 1) * 2;
    $_leapValue = floor(($year - 1) / 4);
    $day1 = floor($year * 0.2422 + $coefficient[$idx1][0]) - $_leapValue;
    if (isset($coefficient[$idx1][1]) && $coefficient[$idx1][1] == $_year) {
        $day1 += $coefficient[$idx1][2];
    }
    $day2 = floor($year * 0.2422 + $coefficient[$idx1 + 1][0]) - $_leapValue;
    if (isset($coefficient[$idx1 + 1][1]) && $coefficient[$idx1 + 1][1] == $_year) {
        $day1 += $coefficient[$idx1 + 1][2];
    }
    $data = [];
    $afterDays = 0;
    if ($day < $day1) {
        $dateTerm = $termName[$idx1 - 1] ?? [];
        $afterDays = $day1 - $day;
    } else {
        if ($day == $day1) {
            $dateTerm = $termName[$idx1] ?? [];
        } else {
            if ($day > $day1 && $day < $day2) {
                $dateTerm = $termName[$idx1] ?? [];
                $afterDays = $day - $day1;
            } else {
                if ($day == $day2) {
                    $dateTerm = $termName[$idx1 + 1] ?? [];
                } else {
                    if ($day > $day2) {
                        $dateTerm = $termName[$idx1 + 1] ?? [];
                        $afterDays = $day - $day2;
                    }
                }
            }
        }
    }
    $data['song'] = '春雨惊春清谷天，夏满芒夏暑相连。秋处露秋寒霜降，冬雪雪冬小大寒。';
    $name = $nameAfter = $dateTerm['name'] ?? '';
    if ($afterDays != 0) {
        $nameAfter = ($dateTerm['name'] ?? '').'后';
    }
    $day1 = $day1 < 10 ? str_pad($day1, 2, 0, STR_PAD_LEFT) : $day1;
    $data['data'] = [
        'date'           => $_year.'-'.$month.'-'.$day1,//节气日期
        'date_timestamp' => strtotime($_year.'-'.$month.'-'.$day1),//节气日期时间戳
        'name'           => $name,//节气名称
        'name_en'        => $dateTerm['name_en'] ?? '',//节气英文
        'name_after'     => $nameAfter,
        'days_after'     => $afterDays,//上一个节气距离天数
        'pic'            => $dateTerm['pic'] ?? '',//节气图片
        'note'           => $dateTerm['note'] ?? '',//节气描述
        'color'          => $dateTerm['color'] ?? '',//节气颜色调
    ];

    return $data;
}

/**
 * 获取poi审核状态值
 *
 * @param $status
 *
 * @return string
 */
function getPoiStatus($status)
{
    $typeArr = ['草稿', '待审核', '不通过', '通过'];

    return $typeArr[$status] ?? '';
}

/**
 * 获取24节气名称
 *
 * @return string[]
 */
function getJieQiArray()
{
    return [
        1  => '立春', 2 => '雨水', 3 => '惊蛰', 4 => '春分', 5 => '清明', 6 => '谷雨', 7 => '立夏', 8 => '小满', 9 => '芒种', 10 => '夏至',
        11 => '小暑', 12 => '大暑', 13 => '立秋', 14 => '处暑', 15 => '白露', 16 => '秋分', 17 => '寒露', 18 => '霜降', 19 => '立冬',
        20 => '小雪', 21 => '大雪', 22 => '冬至', 23 => '小寒', 24 => '大寒',
    ];
}

/**
 * 获取季节,春夏秋冬
 *
 * @param $dateline
 *
 * @return int
 */
function getSeason($dateline)
{
    $queKey = 4;//冬
    $month = date('m', $dateline);
    if (in_array($month, [3, 4, 5])) {
        //春
        $queKey = 1;
    } elseif (in_array($month, [6, 7, 8])) {
        //夏
        $queKey = 2;
    } elseif (in_array($month, [9, 10, 11])) {
        //秋
        $queKey = 3;
    }
    return $queKey;
}

/**
 * 按照气象划分四季
 * 通过时间获取季节对应的节气
 *
 * @param  int  $dateline
 * @param  int  $dateline
 *
 * @return array|string[]
 */
function getSeasonJieQiByDate($dateline = 0)
{
    //春:3,4,5 夏:6,7,8 秋:9,10,11 冬:12,1,2
    $queKeyArr = [
        1 => [
            3 => '惊蛰', 4 => '春分', 5 => '清明', 6 => '谷雨', 7 => '立夏', 8 => '小满', 9 => '芒种',
        ],
        2 => [
            9 => '芒种', 10 => '夏至', 11 => '小暑', 12 => '大暑', 13 => '立秋', 14 => '处暑', 15 => '白露',
        ],
        3 => [
            15 => '白露', 16 => '秋分', 17 => '寒露', 18 => '霜降', 19 => '立冬', 20 => '小雪', 21 => '大雪',
        ],
        4 => [
            21 => '大雪', 22 => '冬至', 23 => '小寒', 24 => '大寒', 1 => '立春', 2 => '雨水', 3 => '惊蛰',
        ],
    ];
    $queKeyJieQiArr = [];
    if ($dateline) {
        $queKey = getSeason($dateline);
        $queKeyJieQiArr = $queKeyArr[$queKey] ?? [];
        //获取当前节气
        $return = getJieQiByDate($dateline);
        $name = $return['data']['name'] ?? '';
        if ( !empty($name)) {
            $jieQiArray = array_flip(getJieQiArray());
            $key = $jieQiArray[$name] ?? '';
            $queKeyJieQiArr[$key] = $name;
        }
        $queKeyJieQiArr = array_unique($queKeyJieQiArr);
    }

    return $queKeyJieQiArr;
}

//
///**
// * 按照气象划分四季
// * 通过时间获取季节对应的节气和时间
// *
// * @param  int  $dateline
// *
// * @return string[]|string[][]
// */
//function getSeasonJieQiByDate($dateline = 0)
//{
//    $datePeriodArr = [];
//    $dateline = empty($dateline) ? time() : $dateline;
//    $month = date('m', $dateline);
//    $year = date('Y', $dateline);
//    $startTime = strtotime($datePeriodArr[0]);
//    $endTime = strtotime($datePeriodArr[1].' 23:59:59');
//    $datePeriodArr = [$startTime, $endTime];
//    if (in_array($month, [3, 4, 5])) {
//        //春
//        $datePeriodArr = [getFristAndLastTimestamp($year,$month)];
//    } elseif (in_array($month, [6, 7, 8])) {
//        //夏
//
//    } elseif (in_array($month, [9, 10, 11])) {
//        //秋
//
//    } else {
//        //冬
//
//    }
//
//
//    return $datePeriodArr;
//}

/**
 *获取指定年月的开始和结束时间戳
 *
 * @param  int  $y  年份
 * @param  int  $m  月份
 *
 * @return array(开始时间,结束时间)
 */
function getFristAndLastTimestamp($y = 0, $m = 0)
{
    $y = $y ?: date('Y');
    $m = $m ?: date('m');
    $d = date('t', strtotime($y.'-'.$m));
    return array("first_time" => strtotime($y.'-'.$m), "last_time" => mktime(23, 59, 59, $m, $d, $y));
}

/*
 * wg
 * 在4或3位的月份,日期添加个分割符,日期格式化
 */
function MDDataForfat($str, $symbol = '-')
{
    if (empty($str)) {
        return '';
    }
    if ( !is_numeric($str)) {
        return false;
    }
    $leng = strlen($str);
    if ($leng < 3 || $leng > 4) {
        return false;
    }
    $temp = str_pad($str, 4, 0, STR_PAD_LEFT);

    return substr($temp, 0, 2).$symbol.substr($temp, 2, 2);
}

/*
 * 开始日期结束日期转节气 月日(0508) 或者完整的日期时间字符串
 */
function dateRangeToJieQi($startDay, $endDay)
{
    if (empty($startDay) || empty($endDay)) {
        return false;
    }
    if (strlen($startDay) <= 4 && strlen($endDay) <= 4) {//非完整的年月日,只是月日(0508)
        $baseY = '2020-';
        $startTime = strtotime($baseY.MDDataForfat($startDay, '-'));
        $endTiime = strtotime($baseY.MDDataForfat($endDay, '-'));
    }
    //节气一般相差15天,这里每隔10天求一次
    $i = 0;
    $jiqieArr = [];
    while (true) {

        $jiqieData = getJieQiByDate($startTime);
        $jiqieArr[] = empty($jiqieData['data']['name']) ? '' : $jiqieData['data']['name'];
        $startTime += 15 * 86400;
        if ($startTime > $endTiime || $i > 10) {
            break;
        }
        $i++;
    }

    return implode(',', array_unique($jiqieArr));
}

/*
 * 中国经纬度四至验证
 * 最东端 东经135度2分30秒（135.04166） 黑龙江和乌苏里江交汇处 最西端 东经73度40分（73.66666666666667） 帕米尔高原乌兹别里山口（乌恰县）
 *  最南端 北纬3度52分（3.8666666666666667） 南沙群岛曾母暗沙 最北端 北纬53度33分（53.5573） 漠河以北黑龙江主航道（漠河县）
 * param $lng 经度,
 * param $lat 维度
 */
function latLngVerify($lng, $lat)
{
    if (empty($lng) || empty($lat) || !is_numeric($lng) || !is_numeric($lat)) {
        return false;
    }
    if ($lng < 73 || $lng > 136) {
        return false;
    }
    if ($lat < 3 || $lat > 54) {
        return false;
    }

    return true;
}

/**
 * 判断单词是否是违禁词
 *
 * @param $word
 *
 * @return bool
 */
function checkDisableWords($word)
{
    if (empty($word)) {
        return false;
    }
    $words = [];
    $wordsArr = config('words.disable') ?? [];
    if (preg_match("/^[a-z]*$/", $word)) {
        $words = $wordsArr['letter'] ?? [];
    } elseif (preg_match("/^[0-9]*$/", $word)) {
        $words = $wordsArr['digital'] ?? [];
    }
    if ($words && in_array($word, $words)) {
        return true;
    }

    return false;
}

/**
 * 获取设置防盗链的视频
 *
 * @param $url string 图片原图路径
 *
 * @return string http://xxx.jpg?_upt=xxx
 */
function getTokenVideo($url)
{
    $url = getUrlByTokenUrl($url);
    if (empty($url)) {
        return $url;
    }
    //获取部分相对路径
    preg_match_all("/http(?:s?):[\/]{2}[a-z\d-]+(?:[.]{1}[a-z\d-]+)+/i", $url, $matches);
    $urlTmp = $matches[0] ?? [];
    if (empty($urlTmp[0])) {
        return $url;
    }
    //图片部分路径path
    $path = str_replace($urlTmp[0], '', $url);
    $upt = getUpyunSign($path, 'a');
    $url = $url . '?_upt=' . $upt;

    return $url;
}

/*
 * 隐私替换  自动识别长度,并替换中间三分之一(最少一个字符)
 * param $string 待替换的字符串
 */
function secretStr($string, $secretStr = '***')
{
    if (empty($string) || !is_string($string)) {
        return false;
    }
    $srtLength = mb_strlen($string, 'utf-8');
    $replaceStart = ceil($srtLength / 3);
    $replacelength = $replaceStart - 1;
    //    $returnStr = substr_replace($string, $secretStr, $replaceStart,($replaceStart+$replacelength));
    $str1 = mb_substr($string, 0, $replaceStart, 'utf-8');
    $str2 = mb_substr($string, ($replaceStart + $replacelength + 1), null, 'utf-8');

    //        var_dump($string);die();
    return $str1.$secretStr.$str2;
}

/*
 * 评论分数小数部分计算(分数只保留0.5或无小数),规则 例如:4.3分计算给4.5分,4.8分计算为5.0分
 */
function commontScoreFormat($score)
{

    $synthesis_score_decimals = $score - floor($score);

    return (empty($synthesis_score_decimals) || $synthesis_score_decimals > 0.5) ? round($score, 0) : (floor($score) + 0.5);//小数部分小于0.5则舍弃,大于0.5则保留0.5
}

/**
 * 获取平台专题类型标识
 *
 * @param $type
 *
 * @return string
 */
function getExTopicTypeSign($type)
{
    $typeArr = [
        1 => 'route',
        2 => 'content',
        3 => 'news',
        4 => 'product',
        5 => 'journey',
        6 => 'scenic',
    ];

    return $typeArr[$type] ?? '';
}

/**
 * plog筛选,获取行业标签
 *
 * @param  string  $type
 *
 * @return string|string[]
 */
function getGalleryType($type = '')
{
    $typeArr = [
        1  => '民宿',
        2  => '景点',
        3  => '导游',
        4  => '特产',
        5  => '美食',
        6  => '旅行社',
        7  => '非遗',
        8  => '文化',
        9  => '体育',
        10 => '旅游',
        11 => '娱乐',
    ];
    if (empty($type)) {
        return $typeArr;
    }
    if (is_array($type)) {
        $arr = [];
        foreach ($type as $item) {
            if ( !empty($typeArr[$item])) {
                $arr[] = $typeArr[$item];
            }
        }
        return $arr;
    }

    return $typeArr[$type] ?? '';
}

/**
 * 通过标识获取行业标签
 * 民宿,景点,导游,特产,美食,非遗,文化体验
 *
 * @param  string|array  $type
 *
 * @return int|mixed
 */
function getGalleryTypeIds($type = '')
{
    $typeArr = [
        1  => '4514724166406412',//民宿
        2  => '4514725211602812',//景点/景区
        3  => '4514725313422919',//导游
        4  => '4514725279924367',//特产
        5  => '4514724081728315',//美食
        6  => '4580828565000299',//旅行社/旅服
        7  => '4514728828503163',//非遗
        8  => '4556412752777262',//文化
        9  => '4600261681717802',//体育
        10 => '4514724480133065',//旅游
        11 => '4600256803848282',//娱乐
    ];
    if (empty($type)) {
        return $typeArr;
    }
    if (is_array($type)) {
        $arr = [];
        foreach ($type as $item) {
            if (mb_strlen($item) == 16) {
                $arr[] = $item;
            } elseif ( !empty($typeArr[$item])) {
                $arr[] = $typeArr[$item];
            }
        }
        return $arr;
    } elseif (mb_strlen($type) == 16) {
        return $type;
    }
    return $typeArr[$type] ?? '';
}

/**
 * 生成随机ID
 *
 * @param $id
 * @param  int  $randCount
 *
 * @return string
 */
function createTableId($id, $randCount = 8)
{
    $id = str_pad($id, 8, 0, STR_PAD_RIGHT);
    $randNum = randomCode($randCount, 'num');

    return $id.$randNum;
}

/**
 *
 * 检测加密后的链接并返回原链接
 *
 * @param $url
 *
 * @return mixed|string
 */
function getUrlByTokenUrl($url)
{
    $urlTmp = '';
    if (empty($url) || !preg_match('/^http(s)?:\\/\\/.+/', $url)) {
        return $urlTmp;
    } elseif (strpos($url, '!exif')) {//图片 exif描述
        return $url;
    } elseif (strpos($url, '!')) {//图片 url
        $urlArr = explode('!', $url);
        $urlTmp = $urlArr[0] ?? '';

        return $urlTmp;
    } elseif (strpos($url, '?')) {//视频 url
        $urlArr = explode('?', $url);
        $urlTmp = $urlArr[0] ?? '';

        return $urlTmp;
    } else {
        $urlTmp = $url;
    }

    return $urlTmp;
}

/**
 * 根据trade_type获取支付方式, 支付方式(0:未知 2:微信,3:支付宝)
 */
function getPaymentWay($trade_type)
{
    return (strpos($trade_type, 'WX') === 0) ? 2 : ((strpos($trade_type, 'ALI') === 0) ? 3 : 0);
}

/**
 * 文创作品 独创码
 *
 * @param $accountId
 * @param $worksId
 *
 * @return string
 */
function getOriginalCode($sn, $accountId, $worksId)
{
    //独创码,1位大写识别号(文创C,豆芽A)+8位账号ID+2位随机数+8位作品ID
    $accountId = str_pad($accountId, 8, 0, STR_PAD_RIGHT);
    $randNum = randomCode(2, 'num');
    $worksId = str_pad($worksId, 8, 0, STR_PAD_LEFT);
    $sn = strtoupper($sn);
    return $sn.$accountId.$randNum.$worksId;
}

/**
 * 获取投诉类型
 *
 * @param  int  $type
 *
 * @return array|string|string[]
 */
function getComplainsType($type = 0)
{
    //投诉类型(1:其他,2:交通票务,3:景区,4:酒店,5:餐厅,6:购物,7:休闲,8:旅行社)
    $typeArr = [
        '1' => '其他', '2' => '交通票务', '3' => '景区', '4' => '酒店', '5' => '餐厅', '6' => '购物', '7' => '休闲', '8' => '旅行社',
    ];
    if (empty($type)) {
        return $typeArr;
    }
    return $typeArr[$type] ?? '';
}

/**
 * 获取投诉状态
 *
 * @param  int  $status
 *
 * @return array|string|string[]
 */
function getComplainsStatus($status = 0)
{
    //投诉类型(1:其他,2:交通票务,3:景区,4:酒店,5:餐厅,6:购物,7:休闲,8:旅行社)
    $statusArr = [
        '1' => '待处理', '2' => '受理中', '3' => '已驳回', '4' => '已处理', '5' => '已撤销',
    ];
    if (empty($status)) {
        return $statusArr;
    }
    return $statusArr[$status] ?? '';
}

/**
 * 获取平台专题内容类型
 * 内容类型(1:民宿,2:地区,3:导游,4:特产,5:美食,6:旅服,7:非遗,8:文化体验)多选,逗号分割
 *
 * @param $type
 *
 * @return array|string
 */
function getExTopicCalssify($type)
{
    $typeArr = [
        1 => '民宿',
        2 => '地区',
        3 => '导游',
        4 => '特产',
        5 => '美食',
        6 => '旅服',
        7 => '非遗',
        8 => '文化体验',
    ];
    if (is_array($type)) {
        $arr = [];
        foreach ($typeArr as $item) {
            if ( !empty($typeArr[$item])) {
                $arr[] = $typeArr[$item];
            }
        }
        return $arr;
    } else {
        return $typeArr[$type] ?? '';
    }
}

/**
 * 百度API, 智能获取图像审核结果
 *
 * @param $token
 * @param $img
 *
 * @return array
 */
function getImageCensor($token, $img)
{
    $response = [];
    $image = $img ?? '';
    if ($image && $token) {
        try {
            $url = 'https://aip.baidubce.com/rest/2.0/solution/v1/img_censor/v2/user_defined?access_token='.$token;
            if ( !preg_match("/data:(.*?);base64,(.*?)$/si", $image)) {
                $image = file_get_contents($image);
                $image = base64_encode($image);
                $body = [
                    'image' => $image,
                ];
            } else {
                $body = [
                    'imgUrl' => $image,
                ];
            }
            $return = curlHttp($url, $body, 'POST');
            if ( !empty($return) && $return['code'] == 200) {
                $response = json_decode($return['response'], true);
            }
        } catch (Exception $e) {
            $msg = $e->getMessage();
            $line = $e->getFile().':'.$e->getLine();
            WriteLog::error(sprintf('智能获取图片[%s]审核结果失败, %s %s', $line, $msg, $img));
        }
    }

    return $response;
}

/**
 * 获取指定时间戳之内的日历
 *
 * @param $start
 * @param $end
 *
 * @return array
 */
function getCalendarByTime($start, $end)
{
    $calendar = [];
    while ($start <= $end) {
        $calendar[] = date('Y-m-d', $start);
        $start = strtotime('+1 day', $start);
    }
    return $calendar;
}

/**
 * 计算两个日期之间相差月份
 *
 * @param $date_end
 * @param $date_start
 * @param  string  $ex
 *
 * @return float|int
 */
function getMonthNum($date_end, $date_start, $ex = '-')
{
    $date_end = explode($ex, $date_end);
    $date_start = explode($ex, $date_start);
    return abs($date_end[0] - $date_start[0]) * 12 + abs($date_end[1] - $date_start[1]);
}

/**
 * 计算两个日期之间相差天数
 *
 * @param $date_end
 * @param $date_start
 *
 * @return false|float
 */
function getDaysNum($date_end, $date_start)
{
    return ceil((strtotime($date_end) - strtotime($date_start)) / 86400);
}

/**
 * 取出最后一个ID
 *
 * @param $areaIds
 *
 * @return mixed|null
 */
function getPopId($ids)
{
    $ids = is_array($ids) ? array_unique(array_filter($ids)) : $ids;
    return array_pop($ids);
}

/**
 * 获取前几个月的月份
 *
 * @param $skip
 *
 * @return array
 */
function getMonthsBySkip($skip)
{
    $month = date('Y-m');
    $months[] = $month;
    for ($i = 1; $i < $skip; $i++) {
        $months[] = date('Y-m', strtotime("-{$i} month"));
    }
    return array_reverse($months);
}

function getMonthsByCondition($start_date, $end_date, $explode = '-')
{
    $start_date = date('Y-m', $start_date);
    $end_date = date('Y-m', $end_date);
    //判断两个时间是不是需要调换顺序
    $start_int = strtotime($start_date);
    $end_int = strtotime($end_date);
    if ($start_int > $end_int) {
        $tmp = $start_date;
        $start_date = $end_date;
        $end_date = $tmp;
    }

    //结束时间月份+1，如果是13则为新年的一月份
    $start_arr = explode($explode, $start_date);
    $start_year = intval($start_arr[0]);
    $start_month = intval($start_arr[1]);

    $end_arr = explode($explode, $end_date);
    $end_year = intval($end_arr[0]);
    $end_month = intval($end_arr[1]);

    $data = array();
    $data[] = date('Y-m', $start_int);

    $tmp_month = $start_month;
    $tmp_year = $start_year;

    //如果起止不相等，一直循环
    while ( !(($tmp_month == $end_month) && ($tmp_year == $end_year))) {
        $tmp_month++;
        //超过十二月份，到新年的一月份
        if ($tmp_month > 12) {
            $tmp_month = 1;
            $tmp_year++;
        }
        $data[] = $tmp_year.$explode.str_pad($tmp_month, 2, '0', STR_PAD_LEFT);
    }
    return $data;
}

/**
 * 获取随机字符串
 *
 * @param $length
 *
 * @return string
 */
function strRandom($length)
{
    //字符组合
    $str = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789';
    $len = strlen($str) - 1;
    $randstr = '';
    for ($i = 0; $i < $length; $i++) {
        $num = mt_rand(0, $len);
        $randstr .= $str[$num];
    }
    return $randstr;
}

/**
 * 获取索引版本
 *
 * @return string
 */
function elasticVersion()
{
    $version = config('elastic.version') ?? '2';
    $version = $version ? '_'.$version : '';
    return config('app.env').$version;
}

//提现费率计算
function cashRate($roleType)
{

    $rate_1 = config('base.cash.rate_1') ?? 0;
    $rate_2 = config('base.cash.rate_2') ?? 0;
    if (in_array($roleType, [3])) {
        $rate = $rate_1;
    } else {
        $rate = $rate_2;
    }
    return $rate;
}

//获取是否后台/中台超管
function roleManage($account)
{
    if (empty($account['type'])) {
        return 0;
    }
    $type = $account['type'] ?? 0;
    $parent_id = $account['parent_id'] ?? '';
    //return 1,为超管
    return ($type == 1 || $type == 2) && $parent_id == '0' ? 1 : 0;
}

/**
 * 将秒数转为天时分秒
 *
 * @param $time
 *
 * @return string
 */
function sec2Time($time)
{
    $st = "";
    if ($time >= 86400) {
        $value['days'] = floor($time / 86400);
        $time = ($time % 86400);
        $st .= $value['days'].'天';
    }
    if ($time >= 3600) {
        $value['hours'] = floor($time / 3600);
        $time = ($time % 3600);
        $st .= $value['hours'].'小时';
    }
    if ($time >= 60) {
        $value['minutes'] = floor($time / 60);
        $time = ($time % 60);
        $st .= $value['minutes'].'分';
    }
    $value['seconds'] = floor($time);
    $st .= $value['seconds'].'秒';
    return $st;
}

/**
 * 将秒数转为年月
 *
 * @param $time
 *
 * @return string
 */
function sec2Year($time)
{
    $st = "";
    if ($time >= 31536000) {
        $value['years'] = floor($time / 31536000);
        $time = ($time % 31536000);
        $st .= $value['years'].'年';
    }
    if ($time >= 2626560) {
        $value['months'] = floor($time / 2626560);
        $time = ($time % 2626560);
        if (empty($st)) {
            $st = $value['months'].'个月';
        } else {
            $st .= '零'.$value['months'].'月';
        }
    }
    return $st;
}

/**
 * @param $str
 * @param $type 1 姓名 2身份证  3手机号
 * @return string
 */
function fieldHide($str, $type)
{
    if (empty($str)) {
        return $str;
    }
    if ($type == 1) { //姓名
        return formatStr($str, 0, 1, '*');
    } elseif ($type == 2) { //身份证
        return formatStr(formatStr($str, 1, 3, '*'), 12, 5, '*');
    } elseif ($type == 3) {//手机号
        return formatStr($str, 3, 4, '*');
    } else {
        return $str;
    }
}

/**
 * 对比金额是否相同
 *
 * @param $num1
 * @param $num2
 *
 * @return bool
 */
function comparisonNumber($num1, $num2)
{
    $num1 = floatval($num1);
    $num2 = floatval($num2);
    return $num1 * 100 == $num2 * 100;
}

/**
 * 获取服务端内网ip地址
 * @return mixed|string
 */
function getServerLocalIp()
{
    $ip = '127.0.0.1';
    $ips = array_values(swoole_get_local_ip());
    foreach ($ips as $v) {
         if ($v && $v != $ip) {
              $ip = $v;
              break;
         }
    }

    return $ip;
}

/**
 * isSupportStdoutLog
 * 判断日志类型是否允许输出
 * @param string $level
 * @return bool
 */
function isSupportStdoutLog(string $level)
{
    $config = config(StdoutLoggerInterface::class, ['log_level' => []]);
    return in_array(strtolower($level), $config['log_level'], true);
}

/**
 * requestEntry
 * 根据异常返回信息，获取请求入口（模块-控制器-方法）
 * @param array $backTrace
 * @return mixed|string
 */
function requestEntry(array $backTrace)
{
    $moduleName = '';
    foreach ($backTrace as $v) {
        if (isset($v['file']) && stripos($v['file'],'CoreMiddleware.php')) {
            $tmp = array_reverse(explode('\\',trim($v['class'])));
            if (substr(strtolower($tmp[0]),-10) == 'controller') {
                $module = str_replace('controller','',strtolower($tmp[1]));
                $class = str_replace('controller','',strtolower($tmp[0]));
                $function = $v['function'];
                $moduleName = $class.'-'.$function;
                if ($module) {
                    $moduleName = $module.'-'.$moduleName;
                }
                break;
            }
        }
    }
    if (!$moduleName) {
        $request = ApplicationContext::getContainer()->get(RequestInterface::class);
        $uri = $request->getRequestUri();
        $moduleName = str_replace('/','-',ltrim($uri,'/'));
    }
    $moduleName = $moduleName??'hyperf';
    return $moduleName;
}

/**
 * getLogArguments
 * 获取要存储的日志部分字段，monolog以外的业务信息
 * @param float $executionTime 程序执行时间，运行时才能判断这里初始化为0
 * @param int $rbs 响应包体大小，初始化0，只有正常请求响应才有值
 * @return array
 */
function getLogArguments($executionTime = null,$rbs = 0)
{
    $request = ApplicationContext::getContainer()->get(RequestInterface::class);
    $requestHeaders = $request->getHeaders();
    $serverParams = $request->getServerParams();
    $arguments = $request->all();
    return [
        'qid' => $requestHeaders['qid'][0]??'',
        'server_name' => $requestHeaders['host'][0]??'',
        'server_addr' => getServerLocalIp()??'',
        'remote_addr' => $serverParams['remote_addr']??'',
        'forwarded_for' => $requestHeaders['x-forwarded-for'][0]??'',
        'user_agent' => $requestHeaders['user-agent'][0]??'',
        'url' => $request->fullUrl()??'',
        'uri' => $serverParams['request_uri']??'',
        'arguments' => $arguments?json_encode($arguments):'',
        'method' => $serverParams['request_method']??'',
        'execution_time' => $executionTime,
        'request_body_size' => $requestHeaders['content-length'][0]??'',
        'response_body_size' => $rbs,
        'referer' => $requestHeaders['referer'][0]??'',
        'unix_time' => $serverParams['request_time']??'',
        'time_day' => isset($serverParams['request_time'])?date('Y-m-d',$serverParams['request_time']):'',
        'time_hour' => isset($serverParams['request_time'])?date('Y-m-d H:00:00',$serverParams['request_time']):'',
    ];
}

function responseDataFormat($code, string $message = '', array $data = []): array
{
    return [
        'return_code' => $code,
        'msg'  => $message,
        'data' => $data,
    ];
}


/**
 * 判断是否为手机号
 * @param $v
 * @return bool
 */
function isMobileNum($v)
{
    $search = '/^0?1[3|4|5|6|7|8][0-9]\d{8}$/';
    if (preg_match($search, $v)) {
        return true;
    } else {
        return false;
    }
}


/**

 * 获取时间戳到毫秒

 * @return bool|string

 */

function getMillisecond(){

    list($msec, $sec) = explode(' ', microtime());

    $msectime =  (float)sprintf('%.0f', (floatval($msec) + floatval($sec)) * 1000);

    return $msectimes = substr($msectime,0,13);

}

function encryptNumber($number) {
    $encryptedString = '';
    # $characters = '0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ';//正常顺序
    $characters = '8zU5F2VqXy1Km4hTdCbJLrG9Ei3QsHnMkRjWpZvD6a7tYgSxAuNwOeIB0fcZ';
    $charactersLength = strlen($characters);
    while ($number > 0) {
        $encryptedString .= $characters[$number % $charactersLength];
        $number = floor($number / $charactersLength);
    }
    return $encryptedString;
}
function decryptNumber($encryptedString) {
    $number = 0;
    # $characters = '0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ';//正常顺序
    $characters = '8zU5F2VqXy1Km4hTdCbJLrG9Ei3QsHnMkRjWpZvD6a7tYgSxAuNwOeIB0fcZ';
    $charactersLength = strlen($characters);
    for ($i = 0; $i < strlen($encryptedString); $i++) {
        $number += strpos($characters, $encryptedString[$i]) * pow($charactersLength, $i);
    }
    return $number;
}