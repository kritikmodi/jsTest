import Suborbital

class Run11: Suborbital.Runnable {
    func run(input: String) -> String {
        return "hello " + input
    }
}

Suborbital.Set(runnable: Run11())
