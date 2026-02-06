package ai.comet.flags

import kotlin.test.Test
import kotlin.test.assertTrue

class AllianceProtocolsTest {

    @Test
    fun engineBootstrapsLedger() {
        val ledger = QuantumLedger()
        val protector = HumanRightsProtector()
        val engine = LegalBanannasEngine(ledger, protector)

        engine.initialize()
        assertTrue(ledger.allEvents().isNotEmpty())
    }
}
