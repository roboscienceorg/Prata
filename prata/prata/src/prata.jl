module prata

# File Path to this file.
LIBRARY = string(@__DIR__,"\\prata")

```
This connect to a master on IP and Port,
The IP will be a string
The Port will be a UInt16
Returns a pointer to the master
```
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

```
This will set the threading option on a master
m will be the object return from connect
thread is a boolean true for turning threading on
```
function setThreading(m, thread)
    @eval ccall(
        (:setThread, #function
        $LIBRARY), #lib
        Cvoid, #return
        (Ptr{Cvoid}, Bool), #input type
        $m,$thread); #input
end

```
This will create a subscriber
m is the object returned from connect
Returns a pointer to the subscriber
```
function subscriber(m)
    return @eval ccall(
        (:subscriberJ, #function
        $LIBRARY), #lib
        Ptr{Cvoid}, #return
        (Ptr{Cvoid},), #input type
        $m) #input
end

```
This will create a publisher
m is the object returned from connect
Returns a pointer to the publisher
```
function publisher(m)
    return @eval ccall(
        (:publisherJ, #function
        $LIBRARY), #lib
        Ptr{Cvoid}, #return
        (Ptr{Cvoid},), #input type
        $m) #input
end

```
This will host a master starting a Prata network
m is the obejct returned from connect
```
function host(m)
    @eval ccall(
        (:hostJ, #function
        $LIBRARY), #lib
        Cvoid, #return
        (Ptr{Cvoid},), #input type
        $m); #input
end

```
This will terminate a master stopping a Prata network
m is the object returned from connect
```
function terminate(m)
    @eval ccall(
        (:terminateJ, #function
        $LIBRARY), #lib
        Cvoid, #return
        (Ptr{Cvoid},), #input type
        $m); #input
end

```
This will return the json of the Prata network m is attached to
this can be used to view what the network looks like without the GUI
m is the objet returned from connect
Returns the json as a string
```
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

```
This will set the port ranges of a Prata network
m is the object return from connect
lower is the lower bounds of the range
upper is the upper bounds of the range
```
function setPortRanges(m, lower, upper)
    @eval ccall(
        (:setThread, #function
        $LIBRARY), #lib
        Cvoid, #return
        (Ptr{Cvoid}, UInt16, UInt16), #input type
        $m, $lower, $upper); #input
end

```
This will create a new channel
m is the object returned from connect
port is the port the channel will be hosted on
style is the style, either FIFO or BROADCAST
messageLimit is the amount of buffer size the channel has
```
function createChannel(m, port, name, style, messageLimit)
     @eval ccall(
        (:createChannelJ, #function
        $LIBRARY), #lib
        Cvoid, #return
        (Ptr{Cvoid}, UInt16, Cstring, Cstring, UInt32), #input type
        $m, $port, $name, $style, $messageLimit); #input
end

```
This will get all available channel types in a Prta network
m is the object returned from connect
```
function getChannelTypes(m)
     @eval ccall(
        (:getChannelTypesJ, #function
        $LIBRARY), #lib
        Cvoid, #return
        (Ptr{Cvoid},), #input type
        $m); #input
end

```
This will connect a publisher to a channel
p is the object returned from publisher
name is the channel to connect to
```
function publisherConnect(p, name)
     @eval ccall(
        (:connectPJ, #function
        $LIBRARY), #lib
        Cvoid, #return
        (Ptr{Cvoid}, Cstring), #input type
        $p, $name); #input
end

```
This will connect a subscriber to a channel
s is the object returned from subscriber
name is the channel to connect to
```
function subscriberConnect(s, name)
     @eval ccall(
        (:connectSJ, #function
        $LIBRARY), #lib
        Cvoid, #return
        (Ptr{Cvoid}, Cstring), #input type
        $s, $name); #input
end

```
This will disconnect a publiser from a channel
p is the object returned from publisher
name is the channel to disconnect from
```
function publisherDisconnect(p, name)
     @eval ccall(
        (:connectPJ, #function
        $LIBRARY), #lib
        Cvoid, #return
        (Ptr{Cvoid}, Cstring), #input type
        $p, $name); #input
end

```
This will disconnect a subscriber from a channel
s is the object returned from subscriber
name is the channel to disconnect from
```
function subscriberDisconnect(s, name)
     @eval ccall(
        (:connectSJ, #function
        $LIBRARY), #lib
        Cvoid, #return
        (Ptr{Cvoid}, Cstring), #input type
        $s, $name); #input
end

```
This will publish data to a channel
p is the publisher that will send the data
chan is the channel name to send to
msg is the data to be sent
```
function publish(p, chan, msg)
     @eval ccall(
        (:publishJ, #function
        $LIBRARY), #lib
        Cvoid, #return
        (Ptr{Cvoid}, Cstring, Cstring), #input type
        $p, $chan, $msg); #input
end

```
This will listen data from a channel
s is the susbcriber that will take the data
chan is the channel name to take from
returning a string for the message contents
```
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
