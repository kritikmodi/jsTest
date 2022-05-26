import Suborbital

class Run13: Suborbital.Runnable {
    func run(input: String) -> String {
        return "hello " + input
    }
}

Suborbital.Set(runnable: Run13())
