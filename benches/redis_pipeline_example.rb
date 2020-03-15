require 'rubygems'
require 'redis'

def bench(descr)
    start = Time.now
    yield
    total_time = Time.new - start
    count_per_second = 100000 / total_time
    puts "#{descr} #{total_time} seconds, #{count_per_second} ops per sec"
end

def without_pipelining
    r = Redis.new
    100000.times {
        r.incr "b"
    }
    100000.times {
      r.decr "b"
    }

    puts "This should be zero: ", (r.get "b")
end

def with_pipelining
    r = Redis.new
    r.pipelined {
        100000.times {
          r.incr "a"
        }
        100000.times {
          r.decr "a"
        }

    }
    puts "This should be zero: ", (r.get "a")
end

bench("without pipelining") {
    without_pipelining
 }
#
bench("with pipelining") {
    with_pipelining
}

