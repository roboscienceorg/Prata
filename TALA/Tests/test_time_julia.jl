using Pkg
Pkg.activate("..\\Build\\Julia\\TALA.jl")
using TALA
using Statistics
using Dates

start = now()

REPEAT = 1000
SLEEP = 2
SPAM = 15

function printStats(stats,title)
    stats = [round(x, digits=5) for x in stats]
    open("timestatsJulia.txt","a") do f
        write(f, string(title, "\n"))
        write(f,"##############\n")
        write(f, string("Min: ", stats[1], "\n"))
        write(f, string("Median: ", stats[2], "\n"))
        write(f, string("Max: ", stats[3], "\n"))
        write(f, string("Average: ", stats[4], "\n"))
        write(f,"##############\n\n")
    end

end

function getStats(times)
    return [minimum(times), median(times), maximum(times), mean(times)]*1000
end



open("timestatsJulia.txt", "w") do f
      write(f, "")
end

println("Startup of Network")
times = []
for i in 1:REPEAT
     tmp = @elapsed begin
        m = TALA.connect("127.0.0.1", 25565)
        TALA.host(m)
        TALA.terminate(m)
    end
    push!(times, tmp)
end

stats = getStats(times)
printStats(stats,"Startup of network")
sleep(SLEEP)


println("Channel creation")
times = []
m = TALA.connect("127.0.0.1", 25565)
TALA.host(m)
s = TALA.subscriber(m)
for i in 1:REPEAT
     tmp = @elapsed begin
        TALA.listen(s,string(rand(Int)))
    end
    push!(times, tmp)
end
stats = getStats(times)
printStats(stats,"Channel creation")
sleep(SLEEP)


println("Listen function call time without known channel")
times = []
TALA.createChannel(m,25566, "NK", "BROADCAST", 1);
for i in 1:REPEAT
    local s = TALA.subscriber(m)
     tmp = @elapsed begin
        TALA.listen(s,"NK")
    end
    push!(times, tmp)
end
stats = getStats(times)
printStats(stats,"Listen function call time without known channel")
sleep(SLEEP)


println("Listen function call time with known channel")
times = []
s = TALA.subscriber(m)
TALA.createChannel(m,25567, "K", "BROADCAST", 1);
TALA.subscriberConnect(s,"K")
for i in 1:REPEAT
     tmp = @elapsed begin
        TALA.listen(s,"K")
    end
    push!(times, tmp)
end
stats = getStats(times)
printStats(stats,"Listen function call time with known channel")
sleep(SLEEP)


println("Publish function call time without known channel")
times = []
for i in 1:REPEAT
    local p = TALA.publisher(m)
     tmp = @elapsed begin
        TALA.publish(p,"NK","yes")
    end
    push!(times, tmp)
end
stats = getStats(times)
printStats(stats,"Publish function call time without known channel")
sleep(SLEEP)


println("Publish function call time with known channel")
times = []
p = TALA.publisher(m)
TALA.publisherConnect(p,"K")
for i in 1:REPEAT
     tmp = @elapsed begin
        TALA.publish(p,"K","yes")
    end
    push!(times, tmp)
end
stats = getStats(times)
printStats(stats,"Publish function call time with known channel")
sleep(SLEEP)


println("Full run without known channel")
times = []
m = TALA.connect("127.0.0.1", 25568)
TALA.host(m)
TALA.subscriberConnect(TALA.subscriber(m),"NK")
for i in 1:REPEAT
    local p = TALA.publisher(m)
    local s = TALA.subscriber(m)
    tmp = @elapsed begin
        TALA.publish(p,"NK","yes")
        TALA.listen(s,"NK")
    end
    push!(times, tmp)
end
stats = getStats(times)
printStats(stats, "Full run without known channel")
sleep(SLEEP)



println("Full run with known channel")
times = []
m = TALA.connect("127.0.0.1", 25569)
TALA.host(m)
p = TALA.publisher(m)
s = TALA.subscriber(m)
TALA.subscriberConnect(s,"K")
TALA.publisherConnect(p,"K")
for i in 1:REPEAT
     tmp = @elapsed begin
        TALA.publish(p,"K","yes")
        TALA.listen(s,"K")
    end
    push!(times, tmp)
end
stats = getStats(times)
printStats(stats, "Full run with known channel")
sleep(SLEEP)



println("Spam Publish")
times = []
m = TALA.connect("127.0.0.1", 25570)
TALA.host(m)
p = TALA.publisher(m)
TALA.publisherConnect(p,"K")
for j in 1:REPEAT
    tmp = @elapsed Threads.@threads for i in 1:SPAM
        local p = TALA.publisher(m)
            TALA.publish(p,"K","yes")
    end
        push!(times, tmp)
end

stats = getStats(times)
printStats(stats,"Spam Publish")
sleep(SLEEP)


println("Spam Listen")
times = []
m = TALA.connect("127.0.0.1", 25570)
s = TALA.subscriber(m)
TALA.subscriberConnect(s,"K")
for j in 1:REPEAT
    tmp = @elapsed Threads.@threads for i in 1:SPAM
        local s = TALA.subscriber(m)
            TALA.listen(s,"K")
    end
        push!(times, tmp)
end

stats = getStats(times)
printStats(stats,"Spam Listen")
sleep(SLEEP)


println("Spam Channel")
cnt = 25570
times = []


for j in 1:REPEAT
    m = TALA.connect("127.0.0.1", cnt)
    TALA.host(m)
    s = TALA.subscriber(m)
    tmp = @elapsed Threads.@threads for i in 1:SPAM
        TALA.subscriberConnect(s,string(rand(Int)))
    end
    TALA.terminate(m)
    global cnt += 1
    push!(times, tmp)
end

stats = getStats(times)
printStats(stats,"Spam Channel")
sleep(SLEEP)


fin = now()

println("Total Time: ", fin-start)
println("Note: ", 12 , " sleeps are used for ", SLEEP ," seconds")
