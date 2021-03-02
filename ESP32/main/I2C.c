void i2cSend(uint8_t addr, uint8_t *data, uint8_t len)
{
  int ret;
  i2c_cmd_handle_t cmd = i2c_cmd_link_create();
  i2c_master_start(cmd);
  for (uint8_t i = 0; i < len; i++)
  {
    i2c_master_write_byte(cmd, data[i], false);
  }
  i2c_master_stop(cmd);
  ret = i2c_master_cmd_begin(I2C_NUM_0, cmd, 0);
  i2c_cmd_link_delete(cmd);
  if (ret != ESP_OK)
  {
    ESP_LOGI(TAG, "I2CError1: %u", ret);
    return ret;
  }
}

void i2cReceive(uint8_t addr, uint8_t *data, uint8_t len)
{
  int ret;
  i2c_cmd_handle_t cmd = i2c_cmd_link_create();
  i2c_master_start(cmd);
  for (uint8_t i = 0; i < len; i++)
  {
    i2c_master_read_byte(cmd, data + i, 0x1);
  }
  i2c_master_stop(cmd);
  ret = i2c_master_cmd_begin(I2C_NUM_0, cmd, 0);
  i2c_cmd_link_delete(cmd);
  if (ret != ESP_OK)
  {
    ESP_LOGI(TAG, "I2CError1: %u", ret);
    return ret;
  }
}

void sh1106_command(uint8_t c)
{
  uint8_t control = 0x00; // Co = 0, D/C = 0
  Wire.beginTransmission(_i2caddr);
  WIRE_WRITE(control);
  WIRE_WRITE(c);
  Wire.endTransmission();
}