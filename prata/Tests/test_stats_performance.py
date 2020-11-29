import prata as tl
import time
import json

print("TEST: checking connection speed to Master Process")
m = tl.connect("127.0.0.1", 25565)

m.host()
#setting up host
time.sleep(1)

sub = m.subscriber()
pub = m.publisher()

#sending messages to 100 channels
start = time.process_time()
i = 0
while i < 25:
    pub.publish(str(i),str(i))
    i += 1

end = time.process_time()

elapsed_time = (end - start) / 100


if(elapsed_time > 20):
    print("---TEST: failure publishing and creating 25 channels took", elapsed_time, "seconds on average")

print("TEST: Testing Pinging Status")
#getting status from master process with 100 channels 100 times
start = time.process_time()
i = 0
while i < 25:
    jsondata = json.loads(m.serialize())
    i+=1
end = time.process_time()

elapsed_time = (end - start) / 100

if(elapsed_time > 1):
    print("---TEST: failure pinging status for 100 channels took", elapsed_time, "seconds on average")


m.terminate()

exit()
