package ai.comet.flags

fun main() {
    val engine = LegalBanannasEngine(
        QuantumLedger(),
        HumanRightsProtector()
    )

    engine.initialize()
    engine.auditEvent("core-policy-circumvention-attempt")
}
