using Pkg
Pkg.activate("..\\Build\\Julia\\TALA.jl")
using TALA

m = TALA.connect("127.0.0.1",25565)

s = TALA.subscriber(m)
p = TALA.publisher(m)

TALA.host(m)

TALA.publish(p,"hello","World")

println(TALA.listen(s,"hello"))

TALA.terminate(m)
