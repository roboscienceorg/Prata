# Import the TALA package
using Pkg
Pkg.activate("..\\Build\\Julia\\TALA.jl")
using TALA

# Connect to a host, this could be run running or one you plan on running
m = TALA.connect("127.0.0.1",25565)

# This is used to launch a host
TALA.host(m)

`
PUBLISHER
`

# Create a publisher
publisher = TALA.publisher(m)

# publish to a channel, if this channel does not exist it will be created
TALA.publish(publisher, "My Channel", "Information")

`
SUBSCRIBER
`

# Create a subscriber
subscriber = TALA.subscriber(m)

# Listen from the channel
data = TALA.listen(subscriber, "My Channel")

println(data)

# Terminate the entire TALA network, stopping all channels.
TALA.terminate(m)
