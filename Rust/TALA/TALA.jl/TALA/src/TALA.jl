module TALA

using PyCall

function __init__()

    try
        py"""
        import TALA
        """
    catch
        pip = pyimport("pip")
        pip.main(["install", "TALA"])
    end

end


function connect(IP, port)
    py"""
    TALA.connect(IP, port)
    """
end

function disconnect(master)
    py"""
    TALA.disconnect(master)
    """
end

function subscriber(channel, master)
    py"""
    TALA.subscriber(channel, master)
    """
end

function publisher(channel, master)
    py"""
    TALA.publisher(channel, master)
    """
end

function gui(master)
    py"""
    TALA.gui(master)
    """
end


end # module
