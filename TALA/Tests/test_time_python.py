import timeit
import time
import numpy as np
import TALA

start = time.time()

REPEATS = 1000
NUMBERS = 1 # will break if >1
TIMEBETWEEN = 2
SPAM = 15
DOSPAM = True

def printStats(stats,title):
    f = open("timestatspython.txt","a")
    stats = [round(x*1000,5) for x in stats]
    f.write(title + '\n')
    f.write("###################\n")
    f.write("Min: " + str(stats[0]) + '\n')
    f.write("Median: " + str(stats[1]) + '\n')
    f.write("Max: " + str(stats[2]) + '\n')
    f.write("Average: " + str(stats[3]) + '\n')
    f.write("###################\n\n")
    f.close()

def getStats(times):
    #print(*([round(x*1000,5) for x in times]))
    return [np.min(times), np.median(times), np.max(times), np.average(times)]

def closeDefault():
    m = TALA.connect("127.0.0.1", 25565)
    m.terminate()

f = open("timestatspython.txt","w")
f.write("")
f.close()

networkStartupSetup='''
import TALA
'''

networkStartup='''
m = TALA.connect("127.0.0.1", 25565)
m.host()
m.terminate()
'''

setupListenNK='''
import TALA
m = TALA.connect("127.0.0.1", 25566)
m.createChannel(25567, "NK", "BROADCAST", 1);
m.publisher().publish("NK","yes")
s = m.subscriber()
'''

setupListenK='''
import TALA
m = TALA.connect("127.0.0.1", 25566)
m.createChannel(25580, "K", "BROADCAST", 1);
m.publisher().publish("K","yes")
s = m.subscriber()
s.connect("K")
'''

setupPublishNK='''
import TALA
m = TALA.connect("127.0.0.1", 25566)
p = m.publisher()
'''

setupPublishK='''
import TALA
m = TALA.connect("127.0.0.1", 25566)
p = m.publisher()
p.connect("K")
'''

createChannel='''
s.listen(str(randint(1,10000)))
'''

print("Startup of network")

printStats(getStats(timeit.Timer(stmt=networkStartup,setup=networkStartupSetup).repeat(number=NUMBERS,repeat=(REPEATS))),"Startup of network")
time.sleep(TIMEBETWEEN)

print("Channel creation")
m = TALA.connect("127.0.0.1", 25565)
m.host()
printStats(getStats(timeit.Timer(stmt=createChannel,setup="import TALA; from random import randint; m = TALA.connect('127.0.0.1', 25565);s=m.subscriber()").repeat(number=NUMBERS,repeat=(REPEATS))),"Channel creation")
m.terminate()
time.sleep(TIMEBETWEEN)

print("Listen function call time without known channel")
m = TALA.connect("127.0.0.1", 25566)
m.host()
printStats(getStats( timeit.Timer(stmt='s.listen("NK")', setup=setupListenNK).repeat(number=NUMBERS,repeat=REPEATS)),"Listen function call time without known channel")
time.sleep(TIMEBETWEEN)

print("Listen function call time with known channel")
printStats(getStats(timeit.Timer(stmt='s.listen("K")', setup=setupListenK).repeat(number=NUMBERS,repeat=REPEATS)),"Listen function call time with known channel")
time.sleep(TIMEBETWEEN)


print("Publish function call time without known channel")
printStats(getStats(timeit.Timer(stmt='p.publish("NK","yes")', setup=setupPublishNK).repeat(number=NUMBERS,repeat=REPEATS)),"Publish function call time without known channel")
time.sleep(TIMEBETWEEN)

print("Publish function call time with known channel")
printStats(getStats(timeit.Timer(stmt='p.publish("K","yes")', setup=setupPublishK).repeat(number=NUMBERS,repeat=REPEATS)),"Publish function call time with known channel")
TALA.connect("127.0.0.1", 25566).terminate()
time.sleep(TIMEBETWEEN)

fullNK='''
import TALA
m = TALA.connect("127.0.0.1", 25568)
p = m.publisher()
s = m.subscriber()
'''

print("Full run without known channel")
m = TALA.connect("127.0.0.1", 25568)
m.host()
m.publisher().publish("NK","yes")
time.sleep(TIMEBETWEEN)
printStats(getStats(timeit.Timer(stmt='p.publish("NK","yes"); s.listen("NK")', setup=fullNK).repeat(number=NUMBERS,repeat=REPEATS)),"Full run without known channel")
m.terminate()
time.sleep(TIMEBETWEEN)


fullK='''
import TALA
m = TALA.connect("127.0.0.1", 25568)
p = m.publisher()
s = m.subscriber()
p.connect("K")
s.connect("K")
'''

print("Full run with known channel")
m = TALA.connect("127.0.0.1", 25568)
m.host()
time.sleep(TIMEBETWEEN)
printStats(getStats(timeit.Timer(stmt='p.publish("K","yes"); s.listen("K")', setup=fullK).repeat(number=NUMBERS,repeat=REPEATS)),"Full run with known channel")
m.terminate()
time.sleep(TIMEBETWEEN)


import threading
if DOSPAM:

    def sendPub(m):
        m.publisher().publish("test","yes")

    def spamPub():
        threads = []
        m = TALA.connect("127.0.0.1",25570)
        for index in range(SPAM):
            x = threading.Thread(target=sendPub, args=(m,))
            threads.append(x)
            x.start()

        for thread in threads:
            thread.join()



    print("Spam Publish")
    m = TALA.connect("127.0.0.1",25570)
    m.host()
    m.publisher().publish("test","yes")
    time.sleep(TIMEBETWEEN)
    printStats(getStats(timeit.Timer(stmt=spamPub).repeat(number=NUMBERS,repeat=(REPEATS))), "Spam Publish")
    m.terminate()
    time.sleep(TIMEBETWEEN)


    def sendLis(m):
        m.subscriber().listen("test")

    def spamLis():
        threads = []
        m = TALA.connect("127.0.0.1",25572)

        for index in range(SPAM):
            x = threading.Thread(target=sendLis, args=(m,))
            threads.append(x)
            x.start()

        for thread in threads:
            thread.join()


    print("Spam Listen")
    m = TALA.connect("127.0.0.1",25572)
    m.host()
    m.publisher().publish("test","yes")
    time.sleep(TIMEBETWEEN)
    printStats(getStats(timeit.Timer(stmt=spamLis).repeat(number=NUMBERS,repeat=(REPEATS))), "Spam Listen")
    m.terminate()
    time.sleep(TIMEBETWEEN)

    from random import randint

    def sendChan(m,chan):

        p = m.publisher()
        s = m.subscriber()
        p.connect(chan)
        s.connect(chan)
        p.publish(chan,"t")
        s.listen(chan)


    def spamChan():
        threads = []
        m = TALA.connect("127.0.0.1",25574)
        tmp = str(randint(1,10000))
        m.publisher().connect(tmp)
        for index in range(SPAM):
            x = threading.Thread(target=sendChan, args=(m, tmp,))
            threads.append(x)
            x.start()

        for thread in threads:
            thread.join()



    print("Spam Channel Creation")
    m = TALA.connect("127.0.0.1",25574)
    m.host()
    time.sleep(TIMEBETWEEN)
    printStats(getStats(timeit.Timer(stmt=spamChan).repeat(number=NUMBERS,repeat=(REPEATS))), "Spam Channel Creation")
    m.terminate()


end = time.time()

print("Total Time: ", (end-start))
print("Note: ", 16 , " sleeps are used for ", TIMEBETWEEN ," seconds")
