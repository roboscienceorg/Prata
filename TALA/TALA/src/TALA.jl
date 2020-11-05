module TALA


function connect(IP, Port)

    tmp = split(IP,".")
    tmp1 = parse(UInt32,tmp[1])
    tmp2 = parse(UInt32,tmp[2])
    tmp3 = parse(UInt32,tmp[3])
    tmp4 = parse(UInt32,tmp[4])
    ip = (tmp1 << 24) + (tmp2 << 16) + (tmp3 << 8) + (tmp4)

    return ccall(
        (:connectJ, #function
        "TALA.dll"), #lib
        Ptr{Cvoid}, #return
        (UInt32, UInt16), #input type
        ip,Port); #input

end

function setThreading(m, thread)
    return ccall(
        (:setThread, #function
        "TALA.dll"), #lib
        Cvoid, #return
        (Ptr{Cvoid}, Bool), #input type
        m,thread); #input
end

function subscriber(m)
    return ccall(
        (:subscriberJ, #function
        "TALA.dll"), #lib
        Ptr{Cvoid}, #return
        (Ptr{Cvoid},), #input type
        m) #input
end

function publisher(m)
    return ccall(
        (:publisherJ, #function
        "TALA.dll"), #lib
        Ptr{Cvoid}, #return
        (Ptr{Cvoid},), #input type
        m) #input
end

function host(m)
    ccall(
        (:hostJ, #function
        "TALA.dll"), #lib
        Cvoid, #return
        (Ptr{Cvoid},), #input type
        m); #input
end

function terminate(m)
    ccall(
        (:terminateJ, #function
        "TALA.dll"), #lib
        Cvoid, #return
        (Ptr{Cvoid},), #input type
        m); #input
end

function serialize(m)
    tmp = ccall(
        (:serializeJ, #function
        "TALA.dll"), #lib
        Cstring, #return
        (Ptr{Cvoid},), #input type
        m);

    ret = unsafe_string(deepcopy(tmp))

    ccall(
        (:freeString, #function
        "TALA.dll"), #lib
        Cvoid, #return
        (Cstring,), #input type
        tmp);

    return ret #input
end

function setPortRanges(m, lower, upper)
    return ccall(
        (:setThread, #function
        "TALA.dll"), #lib
        Cvoid, #return
        (Ptr{Cvoid}, UInt16, UInt16), #input type
        m, lower, upper); #input
end

function createChannel(m, port, name, style, messageLimit)
     ccall(
        (:createChannelJ, #function
        "TALA.dll"), #lib
        Cvoid, #return
        (Ptr{Cvoid}, UInt16, Cstring, Cstring, UInt32), #input type
        m, port, name, style, messageLimit); #input
end

function getChannelTypes(m)
     ccall(
        (:getChannelTypesJ, #function
        "TALA.dll"), #lib
        Cvoid, #return
        (Ptr{Cvoid},), #input type
        m); #input
end

function publisherConnect(p, name)
     ccall(
        (:connectPJ, #function
        "TALA.dll"), #lib
        Cvoid, #return
        (Ptr{Cvoid}, Cstring), #input type
        p, name); #input
end

function subscriberConnect(s, name)
     ccall(
        (:connectSJ, #function
        "TALA.dll"), #lib
        Cvoid, #return
        (Ptr{Cvoid}, Cstring), #input type
        s, name); #input
end

function publisherDisconnect(p, name)
     ccall(
        (:connectPJ, #function
        "TALA.dll"), #lib
        Cvoid, #return
        (Ptr{Cvoid}, Cstring), #input type
        p, name); #input
end

function subscriberDisconnect(s, name)
     ccall(
        (:connectSJ, #function
        "TALA.dll"), #lib
        Cvoid, #return
        (Ptr{Cvoid}, Cstring), #input type
        s, name); #input
end

function publish(p, chan, msg)
     ccall(
        (:publishJ, #function
        "TALA.dll"), #lib
        Cvoid, #return
        (Ptr{Cvoid}, Cstring, Cstring), #input type
        p, chan, msg); #input
end

function listen(s, chan)
        tmp = ccall(
            (:listenJ, #function
            "TALA.dll"), #lib
            Cstring, #return
            (Ptr{Cvoid}, Cstring), #input type
            s, chan);

        ret = unsafe_string(deepcopy(tmp))
        ccall(
            (:freeString, #function
            "TALA.dll"), #lib
            Cvoid, #return
            (Cstring,), #input type
            tmp);

        return ret #input

end
end # module
