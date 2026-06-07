package types


type DeviceSessionDisconnectResponse struct {
	DeviceId string `json:"deviceId"`
	Disconnected bool `json:"disconnected"`
}
