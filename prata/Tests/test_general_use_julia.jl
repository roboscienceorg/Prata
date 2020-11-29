# Import the prata package
using Pkg
Pkg.activate("../Build/Julia/prata.jl")
using prata

# Connect to a host, this could be run running or one you plan on running
m = prata.connect("127.0.0.1",25565)

# This is used to launch a host
prata.host(m)

`
PUBLISHER
`

# Create a publisher
publisher = prata.publisher(m)

# publish to a channel, if this channel does not exist it will be created
prata.publish(publisher, "My Channel", "Information")

`
SUBSCRIBER
`

# Create a subscriber
subscriber = prata.subscriber(m)

# Listen from the channel
data = prata.listen(subscriber, "My Channel")

println(data)

# Terminate the entire prata network, stopping all channels.
prata.terminate(m)
