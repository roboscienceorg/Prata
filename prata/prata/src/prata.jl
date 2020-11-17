module prata

LIBRARY = string(@__DIR__,"\\prata")

function connect(IP, Port)

    tmp = split(IP,".")
    tmp1 = parse(UInt32,tmp[1])
    tmp2 = parse(UInt32,tmp[2])
    tmp3 = parse(UInt32,tmp[3])
    tmp4 = parse(UInt32,tmp[4])
    ip = (tmp1 << 24) + (tmp2 << 16) + (tmp3 << 8) + (tmp4)

    return @eval ccall(
        (:connectJ, #function
        $LIBRARY), #lib
        Ptr{Cvoid}, #return
        (UInt32, UInt16), #input type
        $ip,$Port); #input

end

function setThreading(m, thread)
    return @eval ccall(
        (:setThread, #function
        $LIBRARY), #lib
        Cvoid, #return
        (Ptr{Cvoid}, Bool), #input type
        $m,$thread); #input
end

function subscriber(m)
    return @eval ccall(
        (:subscriberJ, #function
        $LIBRARY), #lib
        Ptr{Cvoid}, #return
        (Ptr{Cvoid},), #input type
        $m) #input
end

function publisher(m)
    return @eval ccall(
        (:publisherJ, #function
        $LIBRARY), #lib
        Ptr{Cvoid}, #return
        (Ptr{Cvoid},), #input type
        $m) #input
end

function host(m)
    @eval ccall(
        (:hostJ, #function
        $LIBRARY), #lib
        Cvoid, #return
        (Ptr{Cvoid},), #input type
        $m); #input
end

function terminate(m)
    @eval ccall(
        (:terminateJ, #function
        $LIBRARY), #lib
        Cvoid, #return
        (Ptr{Cvoid},), #input type
        $m); #input
end

function serialize(m)
    tmp = @eval ccall(
        (:serializeJ, #function
        $LIBRARY), #lib
        Cstring, #return
        (Ptr{Cvoid},), #input type
        $m);

    ret = unsafe_string(deepcopy(tmp))

    @eval ccall(
        (:freeString, #function
        $LIBRARY), #lib
        Cvoid, #return
        (Cstring,), #input type
        $tmp);

    return ret #input
end

function setPortRanges(m, lower, upper)
    return @eval ccall(
        (:setThread, #function
        $LIBRARY), #lib
        Cvoid, #return
        (Ptr{Cvoid}, UInt16, UInt16), #input type
        $m, $lower, $upper); #input
end

function createChannel(m, port, name, style, messageLimit)
     @eval ccall(
        (:createChannelJ, #function
        $LIBRARY), #lib
        Cvoid, #return
        (Ptr{Cvoid}, UInt16, Cstring, Cstring, UInt32), #input type
        $m, $port, $name, $style, $messageLimit); #input
end

function getChannelTypes(m)
     @eval ccall(
        (:getChannelTypesJ, #function
        $LIBRARY), #lib
        Cvoid, #return
        (Ptr{Cvoid},), #input type
        $m); #input
end

function publisherConnect(p, name)
     @eval ccall(
        (:connectPJ, #function
        $LIBRARY), #lib
        Cvoid, #return
        (Ptr{Cvoid}, Cstring), #input type
        $p, $name); #input
end

function subscriberConnect(s, name)
     @eval ccall(
        (:connectSJ, #function
        $LIBRARY), #lib
        Cvoid, #return
        (Ptr{Cvoid}, Cstring), #input type
        $s, $name); #input
end

function publisherDisconnect(p, name)
     @eval ccall(
        (:connectPJ, #function
        $LIBRARY), #lib
        Cvoid, #return
        (Ptr{Cvoid}, Cstring), #input type
        $p, $name); #input
end

function subscriberDisconnect(s, name)
     @eval ccall(
        (:connectSJ, #function
        $LIBRARY), #lib
        Cvoid, #return
        (Ptr{Cvoid}, Cstring), #input type
        $s, $name); #input
end

function publish(p, chan, msg)
     @eval ccall(
        (:publishJ, #function
        $LIBRARY), #lib
        Cvoid, #return
        (Ptr{Cvoid}, Cstring, Cstring), #input type
        $p, $chan, $msg); #input
end

function listen(s, chan)
        tmp = @eval ccall(
            (:listenJ, #function
            $LIBRARY), #lib
            Cstring, #return
            (Ptr{Cvoid}, Cstring), #input type
            $s, $chan);

        ret = unsafe_string(deepcopy(tmp))
        @eval ccall(
            (:freeString, #function
            $LIBRARY), #lib
            Cvoid, #return
            (Cstring,), #input type
            $tmp);

        return ret #input

end
end # module
