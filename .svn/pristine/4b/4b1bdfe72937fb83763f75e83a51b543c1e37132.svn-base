<?php
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

$request = 'method=encrypt&data='.'{"name":"陈莹莹","idCardNo":"342625199307183106","idCardType":"IDENTITY_CARD"}'.'&key=410909cb0ee4462a8364d864ce527b2e';
$result = curlHttp('http://yuy.wgj.qz.gov.cn/api/oauth/JavaAes?'.$request);
var_dump($result);die;
$response = json_decode($result['response'],true);
var_dump($response);die;