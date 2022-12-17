import kotlin.io.path.Path
import kotlin.io.path.forEachLine
import kotlin.math.abs

const val WIDTH = 40
const val HEIGHT = 6

fun main(args: Array<String>) {
    solve1(args)
    solve2(args)
}

fun solve2(args: Array<String>) {
    val path = Path("../input")

    var cycle = 0
    var x = 1

    val drawing = mutableListOf<Boolean>()

    fun incCycles() {
        drawing.add(abs(x - (cycle % WIDTH)) <= 1)
        cycle += 1
    }

    path.forEachLine {line ->
        val instruction = line.split(" ")
        if (instruction[0] == "noop") {
            incCycles()
        } else if (instruction[0] == "addx") {
            incCycles()
            incCycles()
            x += instruction[1].toInt()
        }
    }

    println("ans2")
    for (i in 0 until HEIGHT) {
        for (j in 0 until WIDTH) {
            val index = i * WIDTH + j
            print(if (drawing[index]) '#' else '.')
        }
        println()
    }
}

fun solve1(args: Array<String>) {
    val path = Path("../input")

    var cycle = 0
    var x = 1

    var sum = 0

    fun incCycles() {
        cycle += 1
        if ((cycle + 20) % 40 == 0) {
            sum += cycle * x
        }
    }
    path.forEachLine {line ->
        val instruction = line.split(" ")
        if (instruction[0] == "noop") {
            incCycles()
        } else if (instruction[0] == "addx") {
            incCycles()
            incCycles()
            x += instruction[1].toInt()
        }
    }

    println("ans1 $sum")
}