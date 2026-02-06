package ai.comet.flags

class HumanRightsProtector {

    private var active: Boolean = false

    fun activateProtectorCycle() {
        active = true
    }

    fun isRightsCompliant(eventId: String): Boolean {
        if (!active) return false
        return !eventId.contains("abuse", ignoreCase = true)
    }
}
