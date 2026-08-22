<?php

declare(strict_types=1);

return [
    'endpoint' => env('S3_ENDPOINT'),
    'key' => env('S3_ACCESS_KEY'),
    'secret' => env('S3_SECRET_KEY'),
    'bucket' => env('S3_BUCKET'),
    'region' => env('S3_REGION'),
    'prefix' => env('S3_PREFIX', 'ticket-file'),
    'acl' => env('S3_ACL', 'public-read'),
];
