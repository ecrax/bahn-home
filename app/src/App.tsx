import type { Journey } from "./bindings";
import { rspc } from "./rspc";

function App() {
	const { data: dbInfo, isLoading: dbLoading } = rspc.useQuery(["db"]);
	const { data: kvbInfo, isLoading: kvbLoading } = rspc.useQuery(["kvb"]);

	return (
		<div className="rounded-xl text-white">
			{dbLoading || kvbLoading ? (
				<div className="w-screen h-screen flex justify-center items-center">Loading...</div>
			) : (
				<div>
					<TrainSegment journey={dbInfo} color="green" />
					<TrainSegment journey={kvbInfo} color="red" />
				</div>
			)}
		</div>
	);
}

const TrainSegment: React.FC<{ journey: Journey; color: "green" | "red" }> = ({
	journey,
	color,
}) => {
	const duration =
		new Date(journey.planned_arrival).getTime() -
		new Date(journey.planned_depature).getTime();
	const arrivalDelay = new Date(
		new Date(journey.arrival).getTime() -
			new Date(journey.planned_arrival).getTime(),
	).getMinutes();
	const departureDelay = new Date(
		new Date(journey.departure).getTime() -
			new Date(journey.planned_depature).getTime(),
	).getMinutes();

	return (
		<div className="rounded-lg text-white flex ring-1 ring-gray-700 shadow-md m-4 p-4 items-center">
			<div className="flex flex-col pr-4">
				<p>
					{new Date(journey.planned_depature).toLocaleTimeString("de-DE", {
						hour: "2-digit",
						minute: "2-digit",
					})}
				</p>
				{departureDelay > 0 && (
					<p
						className={`text-sm ${departureDelay > 5 ? "text-red-400" : "text-green-400"}`}
					>
						{new Date(journey.departure).toLocaleTimeString("de-DE", {
							hour: "2-digit",
							minute: "2-digit",
						})}
					</p>
				)}

				<p className="text-xs max-w-24 text-ellipsis whitespace-nowrap overflow-hidden">
					{journey.from}
				</p>
			</div>
			<div
				className={`h-0.5 flex-grow ${color === "green" ? "bg-green-700" : "bg-red-700"}`}
			/>
			<div className="p-4 text-center text-xs">
				<p>{new Date(duration).getMinutes()}min </p>
				<p>{journey.line}</p>
			</div>
			<div
				className={`h-0.5 flex-grow ${color === "green" ? "bg-green-700" : "bg-red-700"}`}
			/>
			<div className="flex flex-col text-right pl-4">
				<p>
					{new Date(journey.planned_arrival).toLocaleTimeString("de-DE", {
						hour: "2-digit",
						minute: "2-digit",
					})}
				</p>
				{arrivalDelay > 0 && (
					<p
						className={`text-sm ${arrivalDelay > 5 ? "text-red-400" : "text-green-400"}`}
					>
						{new Date(journey.arrival).toLocaleTimeString("de-DE", {
							hour: "2-digit",
							minute: "2-digit",
						})}
					</p>
				)}
				<p className="text-xs max-w-24 text-ellipsis whitespace-nowrap overflow-hidden">
					{journey.to}
				</p>
			</div>
		</div>
	);
};

export default App;
