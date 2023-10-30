package org.entur.platform.examples.service.greet

import org.assertj.core.api.Assertions.assertThat
import org.junit.jupiter.api.Test
import org.springframework.beans.factory.annotation.Autowired
import org.springframework.boot.test.context.SpringBootTest
import org.springframework.boot.test.web.client.TestRestTemplate
import org.springframework.boot.web.server.LocalServerPort

@SpringBootTest(webEnvironment = SpringBootTest.WebEnvironment.RANDOM_PORT)
class HelloWorldTests {

    @Autowired
    lateinit var restTemplate: TestRestTemplate

    @LocalServerPort
    var port: Int = 0

    @Test
    fun `returns "Hello World" as default`() {
        val responseEntity = restTemplate.getForEntity("http://localhost:" + port + "/greet", Map::class.java)
        assertThat(responseEntity.statusCodeValue).isEqualTo(200)
        assertThat(responseEntity.body!!["greeting"]).isEqualTo("Hello World")
    }

    @Test
    fun `returns name from query parameter when supplied`() {
        val name = "Jones"
        val responseEntity = restTemplate.getForEntity("http://localhost:" + port + "/greet?name={name}", Map::class.java, name)
        assertThat(responseEntity.statusCodeValue).isEqualTo(200)
        assertThat(responseEntity.body!!["greeting"]).isEqualTo("Hello " + name)
    }

}
