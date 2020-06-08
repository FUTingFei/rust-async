# rust-async
虚拟机:
内存 13 G 
处理器 8 C
```shell
flyq@ubuntu:~$ ab -n 100000 -c 400 http://127.0.0.1:3000/
This is ApacheBench, Version 2.3 <$Revision: 1807734 $>
Copyright 1996 Adam Twiss, Zeus Technology Ltd, http://www.zeustech.net/
Licensed to The Apache Software Foundation, http://www.apache.org/

Benchmarking 127.0.0.1 (be patient)
Completed 10000 requests
Completed 20000 requests
Completed 30000 requests
Completed 40000 requests
Completed 50000 requests
Completed 60000 requests
Completed 70000 requests
Completed 80000 requests
Completed 90000 requests
Completed 100000 requests
Finished 100000 requests


Server Software:        
Server Hostname:        127.0.0.1
Server Port:            3000

Document Path:          /
Document Length:        13 bytes

Concurrency Level:      400
Time taken for tests:   5.901 seconds
Complete requests:      100000
Failed requests:        0
Total transferred:      8900000 bytes
HTML transferred:       1300000 bytes
Requests per second:    16945.90 [#/sec] (mean)
Time per request:       23.605 [ms] (mean)
Time per request:       0.059 [ms] (mean, across all concurrent requests)
Transfer rate:          1472.84 [Kbytes/sec] received

Connection Times (ms)
              min  mean[+/-sd] median   max
Connect:        0   13  81.4      6    1037
Processing:     1   10   4.8     10     221
Waiting:        1    8   4.4      7     218
Total:          2   23  82.3     16    1231

Percentage of the requests served within a certain time (ms)
  50%     16
  66%     19
  75%     20
  80%     21
  90%     23
  95%     24
  98%     27
  99%     30
 100%   1231 (longest request)
```