<?php

declare(strict_types=1);

return [
    'sms_driver' => env('LEGACY_SMS_DRIVER'),
    'app_downloadurl' => env('LEGACY_APP_DOWNLOAD_URL'),
    'alisms' => [
        'regionId' => env('LEGACY_ALISMS_REGION_ID'),
        'endPointName' => env('LEGACY_ALISMS_ENDPOINT_NAME'),
        'accesskey' => env('LEGACY_ALISMS_ACCESS_KEY'),
        'accessSecret' => env('LEGACY_ALISMS_ACCESS_SECRET'),
        'signname' => env('LEGACY_ALISMS_SIGN_NAME'),
    ],
    'gdusms' => [
        'operId' => env('LEGACY_GDUSMS_OPERATOR_ID'),
        'operPass' => env('LEGACY_GDUSMS_OPERATOR_PASSWORD'),
    ],
    'lxtsms' => [
        'User' => env('LEGACY_LXTSMS_USER'),
        'Password' => env('LEGACY_LXTSMS_PASSWORD'),
        'CorpId' => env('LEGACY_LXTSMS_CORP_ID'),
    ],
];
