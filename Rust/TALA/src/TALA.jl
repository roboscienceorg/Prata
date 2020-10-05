module TALA
    using PyCall

    function __init__()
        try
            TALA = pyimport("TALA")
        catch err
            println("Please enter location of TALA wheel file: ")
            pip = pyimport("pip")
            pip.main(["install", readline()])
            TALA = pyimport("TALA")
        end

    end



end # module
