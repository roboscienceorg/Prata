import prata
import time
import timeit
import numpy as np
import sys
import random
import string

B = 1
KB = 1000
MB = 1000000

END = []

def getString(s):
    #''.join(random.choice(string.ascii_letters) for x in range(int(real)))
    real  = 1 if (s-49) < 1 else s-49
    return "1"*real

TIMES = 5
SLEEP = 2
NUMBER = 1

def printStats(stats, title, size, u = True):
    f = open("pythonThrouput.txt","a")

    stats = [round((x)/NUMBER,5) for x in stats]
    kilo = size#/1000
    bytes = "B"


    if u:
        if kilo > 1000:
            kilo /= 1000
            bytes = "KB"
        if kilo > 1000:
            kilo /= 1000
            bytes = "MB"
        mbs = [round(kilo/(i),5) for i in stats]
        f.write(title + '\n')
        f.write("###################\n")
        f.write("Min: {}s\n".format(stats[0]))
        f.write("Min {}/s: {}\n".format(bytes,mbs[0]))
        f.write("Median: {}s\n".format(stats[1]))
        f.write("Median {}/s: {}\n".format(bytes,mbs[1]))
        f.write("Max: {}s\n".format(stats[2]))
        f.write("Max {}/s: {}\n".format(bytes,mbs[2]))
        f.write("Average: {}s\n".format(stats[3]))
        f.write("Average {}/s: {}\n".format(bytes,mbs[3]))
        f.write("###################\n\n")
    else:
        END.append([kilo,stats[3]])
    f.close()


def getStats(times):
    return [np.min(times), np.median(times), np.max(times), np.average(times)]

f = open("pythonThrouput.txt","w")
f.close()

m = prata.connect("127.0.0.1",25565)
m.host()
m.createChannel(25566, "test", "FIFO", 500)

SIZE = (1*B)
testListen='''
p.publish("test",STRING)
t = s.listen("test")
'''
obj = getString(SIZE)
setupTestListen = """
STRING =  \""""+obj+"""\"
import prata
m = prata.connect(\"127.0.0.1\",25565)
s = m.subscriber()
p = m.publisher()
s.connect(\"test\")
p.connect(\"test\")
"""

printStats(getStats(timeit.Timer(stmt=testListen,setup=setupTestListen).repeat(number=NUMBER,repeat=TIMES)),"Round Trip 50 bytes",  sys.getsizeof(obj))



SIZE = (1*KB)
testListen='''
p.publish("test",STRING)
t = s.listen("test")
'''

obj = getString(SIZE)

setupTestListen = """
STRING =  \""""+obj+"""\"
import prata
m = prata.connect(\"127.0.0.1\",25565)
s = m.subscriber()
p = m.publisher()
s.connect(\"test\")
p.connect(\"test\")
"""

printStats(getStats(timeit.Timer(stmt=testListen,setup=setupTestListen).repeat(number=NUMBER,repeat=TIMES)),"Round Trip 1 KiloByte", sys.getsizeof(obj))



SIZE = (1*MB)
testListen='''
p.publish("test",STRING)
t = s.listen("test")
'''

obj = getString(SIZE)

setupTestListen = """
STRING =  \""""+obj+"""\"
import prata
m = prata.connect(\"127.0.0.1\",25565)
s = m.subscriber()
p = m.publisher()
s.connect(\"test\")
p.connect(\"test\")
"""

printStats(getStats(timeit.Timer(stmt=testListen,setup=setupTestListen).repeat(number=NUMBER,repeat=TIMES)),"Round Trip 1 MegaByte", sys.getsizeof(obj))



SIZE = (35*MB)
testListen='''
p.publish("test",STRING)
t = s.listen("test")
'''

obj = getString(SIZE)

setupTestListen = """
STRING =  \""""+obj+"""\"
import prata
m = prata.connect(\"127.0.0.1\",25565)
s = m.subscriber()
p = m.publisher()
s.connect(\"test\")
p.connect(\"test\")
"""

printStats(getStats(timeit.Timer(stmt=testListen,setup=setupTestListen).repeat(number=NUMBER,repeat=TIMES)),"Round Trip 35 MegaBytes", sys.getsizeof(obj))








def largegroup():
    for i in range(1,10001):
        SIZE = i*100
        testListen='''
p.publish("test",STRING)
t = s.listen("test")
        '''

        obj = getString(SIZE)

        setupTestListen = """
STRING =  \""""+obj+"""\"
import prata
m = prata.connect(\"127.0.0.1\",25565)
s = m.subscriber()
p = m.publisher()
s.connect(\"test\")
p.connect(\"test\")
        """

        printStats(getStats(timeit.Timer(stmt=testListen,setup=setupTestListen).repeat(number=1,repeat=2)),"Round Trip {} Bytes".format(SIZE), sys.getsizeof(obj),False)
        time.sleep(.1)



def manyGroup():
        for i in range(1,1001):
            R = random.randint(1,10)
            SIZE = i*100
            testListen='''
for i in range('''+str(R)+'''):
    p.publish("test",STRING)
for i in range('''+str(R)+'''):
    t = s.listen("test")
            '''

            obj = getString(SIZE)

            setupTestListen = """
STRING =  \""""+obj+"""\"
import prata
m = prata.connect(\"127.0.0.1\",25565)
s = m.subscriber()
p = m.publisher()
s.connect(\"test\")
p.connect(\"test\")
            """

            printStats(getStats(timeit.Timer(stmt=testListen,setup=setupTestListen).repeat(number=1,repeat=2)),"".format(SIZE), sys.getsizeof(obj),False)
            END[-1].append(R)
            time.sleep(.1)


manyGroup()
f = open("many.csv","w")
f.write("Bytes,Time(s),Iterations,Total Throughput, Singular Throughput\n")
for e in END:
    f.write(str(e[0]))
    f.write(", ")
    f.write(str(e[1]))
    f.write(", ")
    f.write(str(e[2]))
    f.write(", ")
    f.write(str(e[0]/e[1]))
    f.write(", ")
    f.write(str((e[0]/e[1])*e[2]))
    f.write("\n")

f.close()
m.terminate()
