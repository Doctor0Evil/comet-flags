package ai.comet.flags

class QuantumLedger {

    private val events: MutableList<String> = mutableListOf()

    fun bootstrapAnchorage() {
        events.add("bootstrap-ledger")
    }

    fun recordViolation(id: String) {
        events.add("violation:$id")
    }

    fun allEvents(): List<String> = events.toList()
}
