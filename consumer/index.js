import { Kafka } from 'kafkajs'

const getTime = () => {
	let date = new Date()

	return `${date.getHours()}:${date.getMinutes()}:${date.getSeconds()}`
}

const kafka = new Kafka({
	clientId: 'Prism',
	brokers: ['localhost:9092'],
})

const producer = kafka.producer()
const consumer = kafka.consumer({
	groupId: 'general-purpose-3',
})

await producer.connect()
await consumer.connect()

await consumer.subscribe({ topic: 'general-forth', fromBeginning: true })

await consumer
	.run({
		eachMessage: async ({ partition, message }) => {
			console.log({
				partition,
				offset: message.offset,
				value: message.value.toString(),
			})

			await new Promise((resolve) => setTimeout(resolve, 1000))

			await producer.send({
				topic: 'general-back',
				messages: [
					{
						value: getTime(),
					},
				],
			})
		},
	})
	.catch((err) => {
		console.warn('Err:', err)
	})
