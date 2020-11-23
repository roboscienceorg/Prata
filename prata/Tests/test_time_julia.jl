using Pkg
Pkg.activate("..\\Build\\Julia\\prata.jl")
using prata
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
        m = prata.connect("127.0.0.1", 25565)
        prata.host(m)
        prata.terminate(m)
    end
    push!(times, tmp)
end

stats = getStats(times)
printStats(stats,"Startup of network")
sleep(SLEEP)


println("Channel creation")
times = []
m = prata.connect("127.0.0.1", 25565)
prata.host(m)
s = prata.subscriber(m)
for i in 1:REPEAT
     tmp = @elapsed begin
        prata.listen(s,string(rand(Int)))
    end
    push!(times, tmp)
end
stats = getStats(times)
printStats(stats,"Channel creation")
sleep(SLEEP)


println("Listen function call time without known channel")
times = []
prata.createChannel(m,25566, "NK", "BROADCAST", 1);
for i in 1:REPEAT
    local s = prata.subscriber(m)
     tmp = @elapsed begin
        prata.listen(s,"NK")
    end
    push!(times, tmp)
end
stats = getStats(times)
printStats(stats,"Listen function call time without known channel")
sleep(SLEEP)


println("Listen function call time with known channel")
times = []
s = prata.subscriber(m)
prata.createChannel(m,25567, "K", "BROADCAST", 1);
prata.subscriberConnect(s,"K")
for i in 1:REPEAT
     tmp = @elapsed begin
        prata.listen(s,"K")
    end
    push!(times, tmp)
end
stats = getStats(times)
printStats(stats,"Listen function call time with known channel")
sleep(SLEEP)


println("Publish function call time without known channel")
times = []
for i in 1:REPEAT
    local p = prata.publisher(m)
     tmp = @elapsed begin
        prata.publish(p,"NK","yes")
    end
    push!(times, tmp)
end
stats = getStats(times)
printStats(stats,"Publish function call time without known channel")
sleep(SLEEP)


println("Publish function call time with known channel")
times = []
p = prata.publisher(m)
prata.publisherConnect(p,"K")
for i in 1:REPEAT
     tmp = @elapsed begin
        prata.publish(p,"K","yes")
    end
    push!(times, tmp)
end
stats = getStats(times)
printStats(stats,"Publish function call time with known channel")
sleep(SLEEP)


println("Full run without known channel")
times = []
m = prata.connect("127.0.0.1", 25568)
prata.host(m)
prata.subscriberConnect(prata.subscriber(m),"NK")
for i in 1:REPEAT
    local p = prata.publisher(m)
    local s = prata.subscriber(m)
    tmp = @elapsed begin
        prata.publish(p,"NK","yes")
        prata.listen(s,"NK")
    end
    push!(times, tmp)
end
stats = getStats(times)
printStats(stats, "Full run without known channel")
sleep(SLEEP)



println("Full run with known channel")
times = []
m = prata.connect("127.0.0.1", 25569)
prata.host(m)
p = prata.publisher(m)
s = prata.subscriber(m)
prata.subscriberConnect(s,"K")
prata.publisherConnect(p,"K")
for i in 1:REPEAT
     tmp = @elapsed begin
        prata.publish(p,"K","yes")
        prata.listen(s,"K")
    end
    push!(times, tmp)
end
stats = getStats(times)
printStats(stats, "Full run with known channel")
sleep(SLEEP)



println("Spam Publish")
times = []
m = prata.connect("127.0.0.1", 25570)
prata.host(m)
p = prata.publisher(m)
prata.publisherConnect(p,"K")
for j in 1:REPEAT
    tmp = @elapsed Threads.@threads for i in 1:SPAM
        local p = prata.publisher(m)
            prata.publish(p,"K","yes")
    end
        push!(times, tmp)
end

stats = getStats(times)
printStats(stats,"Spam Publish")
sleep(SLEEP)


println("Spam Listen")
times = []
m = prata.connect("127.0.0.1", 25570)
s = prata.subscriber(m)
prata.subscriberConnect(s,"K")
for j in 1:REPEAT
    tmp = @elapsed Threads.@threads for i in 1:SPAM
        local s = prata.subscriber(m)
            prata.listen(s,"K")
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
    m = prata.connect("127.0.0.1", cnt)
    prata.host(m)
    s = prata.subscriber(m)
    tmp = @elapsed Threads.@threads for i in 1:SPAM
        prata.subscriberConnect(s,string(rand(Int)))
    end
    prata.terminate(m)
    global cnt += 1
    push!(times, tmp)
end

stats = getStats(times)
printStats(stats,"Spam Channel")
sleep(SLEEP)


fin = now()

println("Total Time: ", fin-start)
println("Note: ", 12 , " sleeps are used for ", SLEEP ," seconds")
