import Suborbital

class Run14: Suborbital.Runnable {
    func run(input: String) -> String {
        return "hello " + input
    }
}

Suborbital.Set(runnable: Run14())
