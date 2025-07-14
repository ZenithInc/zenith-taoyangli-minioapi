<?php

namespace App\Core\Common\Handler;

use Monolog\Handler\StreamHandler;
use Monolog\Logger;

class LogFileHandler extends  StreamHandler
{

    public function handle(array $record):bool
    {
        if (!$this->isHandling($record)) {
            return false;
        }
        $record = $this->processRecord($record);
        if ( !config('log_enable') ) {
            return false;
        }
        if ( !config('hf_log') && $record['channel'] == 'hyperf' ) {
            return false;
        }
        if ( ! isSupportStdoutLog($record['level_name']) ) {
            return false;
        }
        $record['formatted'] = $this->getFormatter()->format($record);
        $this->write($record);
        return false === $this->bubble;
    }


    public function isHandling(array $record):bool
    {
        switch ($record['level']) {
            case Logger::DEBUG:
                return $record['level'] == $this->level;
                break;
            case $record['level'] == Logger::ERROR || $record['level'] == Logger::CRITICAL || $record['level'] == Logger::ALERT || $record['level'] == Logger::EMERGENCY:
                return Logger::ERROR <= $this->level && Logger::EMERGENCY >= $this->level;
                break;
            default:
                return Logger::INFO <= $this->level && Logger::WARNING >= $this->level;
        }
    }


}