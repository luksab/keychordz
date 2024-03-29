/* This example code is in the Public Domain (or CC0 licensed, at your option.)
   Unless required by applicable law or agreed to in writing, this software is 
   distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR  
   CONDITIONS OF ANY KIND, either express or implied.
*/
#include "ble_import.c"

#define LEFT_KEY 18
#define RIGHT_KEY 15

void hid_demo_task(void *pvParameters)
{
    vTaskDelay(1000 / portTICK_PERIOD_MS);
    while (1)
    {
        vTaskDelay(20 / portTICK_PERIOD_MS);
        if (sec_conn)
        {

            send_volum_up = true;
            //uint8_t *key_values = (uint8_t*)malloc(2);
            uint8_t key_values[2];
            uint8_t num_keys = 0;

            //printf("%d %d\n", gpio_get_level(LEFT_KEY), gpio_get_level(RIGHT_KEY));

            if (!gpio_get_level(LEFT_KEY))
            {
                //ESP_LOGI(HID_DEMO_TAG, "Send RIGHT");
                key_values[num_keys++] = HID_KEY_LEFT_ARROW;
            }
            if (!gpio_get_level(RIGHT_KEY))
            {
                //ESP_LOGI(HID_DEMO_TAG, "Send LEFT");
                key_values[num_keys++] = HID_KEY_RIGHT_ARROW;
            }

            esp_hidd_send_keyboard_value(hid_conn_id, 0, (uint8_t *)&key_values, num_keys);
            //esp_hidd_send_mouse_value(hid_conn_id, 0, 0, 10);

            // esp_hidd_send_consumer_value(hid_conn_id, HID_CONSUMER_VOLUME_UP, true);
            // vTaskDelay(3000 / portTICK_PERIOD_MS);
            // if (send_volum_up) {
            //     send_volum_up = false;
            //     esp_hidd_send_consumer_value(hid_conn_id, HID_CONSUMER_VOLUME_UP, false);
            //     esp_hidd_send_consumer_value(hid_conn_id, HID_CONSUMER_VOLUME_DOWN, true);
            //     vTaskDelay(3000 / portTICK_PERIOD_MS);
            //     esp_hidd_send_consumer_value(hid_conn_id, HID_CONSUMER_VOLUME_DOWN, false);
            // }
        }
    }
}

void setupGpio()
{
    gpio_set_direction(RIGHT_KEY, GPIO_MODE_INPUT);
    gpio_set_direction(LEFT_KEY, GPIO_MODE_INPUT);
    gpio_pullup_en(RIGHT_KEY);
    gpio_pullup_en(LEFT_KEY);
}

#include <u8g2.h>
#include <u8g2_esp32_hal.h>

// CLK - GPIO14
#define PIN_CLK 14

// MOSI - GPIO 13
#define PIN_MOSI 13

// RESET - GPIO 26
#define PIN_RESET 27

// DC - GPIO 27
#define PIN_DC 26

// CS - GPIO 15
#define PIN_CS 25
static char tag[] = "test_SSD1306";

void task_test_SSD1306()
{
    ESP_LOGI(tag, "Starting test");
    u8g2_esp32_hal_t u8g2_esp32_hal = U8G2_ESP32_HAL_DEFAULT;
    // u8g2_esp32_hal.clk = PIN_CLK;
    // u8g2_esp32_hal.mosi = PIN_MOSI;
    // u8g2_esp32_hal.cs = PIN_CS;
    // u8g2_esp32_hal.dc = PIN_DC;
    // u8g2_esp32_hal.reset = PIN_RESET;
    u8g2_esp32_hal.sda = PIN_CLK;
    u8g2_esp32_hal.scl = PIN_MOSI;
    
    u8g2_esp32_hal_init(u8g2_esp32_hal);

    u8g2_t u8g2; // a structure which will contain all the data for one display
    // u8g2_Setup_ssd1306_128x64_noname_f(
    //     &u8g2,
    //     U8G2_R0,
    //     u8g2_esp32_spi_byte_cb,
    //     u8g2_esp32_gpio_and_delay_cb); // init u8g2 structure

    u8g2_Setup_sh1106_128x64_noname_2(
        &u8g2,
        U8G2_R1,
        u8g2_esp32_i2c_byte_cb,
        u8g2_esp32_gpio_and_delay_cb); // init u8g2 structure

    u8g2_InitDisplay(&u8g2); // send init sequence to the display, display is in sleep mode after this,

    u8g2_SetPowerSave(&u8g2, 0); // wake up display
    vTaskDelay(10);
    u8g2_ClearBuffer(&u8g2);
    //u8g2_DrawBox(&u8g2, 10,20, 20, 30);
    u8g2_SetFont(&u8g2, u8g2_font_ncenB14_tr);
    u8g2_DrawStr(&u8g2, 1, 30, "Hello World!");
    u8g2_SendBuffer(&u8g2);
    vTaskDelay(100);

    ESP_LOGI(tag, "All done!");

    uint counter = 0;
    while (0)
    {
        //u8g2_SetPowerSave(&u8g2, 0); // wake up display
        u8g2_ClearBuffer(&u8g2);
        //u8g2_DrawBox(&u8g2, 10,20, 20, 30);
        //u8g2_SetFont(&u8g2, u8g2_font_ncenB14_tr);
        char text[32];
        sprintf(text, "%d", counter++);
        u8g2_DrawStr(&u8g2, 1, 15, text);
        u8g2_SendBuffer(&u8g2);
        // if (!(counter % 100))
            vTaskDelay(100);
    }

    vTaskDelete(NULL);
}

void app_main(void)
{
    esp_err_t ret;

    // Initialize NVS.
    ret = nvs_flash_init();
    if (ret == ESP_ERR_NVS_NO_FREE_PAGES || ret == ESP_ERR_NVS_NEW_VERSION_FOUND)
    {
        ESP_ERROR_CHECK(nvs_flash_erase());
        ret = nvs_flash_init();
    }
    ESP_ERROR_CHECK(ret);

    ESP_ERROR_CHECK(esp_bt_controller_mem_release(ESP_BT_MODE_CLASSIC_BT));

    esp_bt_controller_config_t bt_cfg = BT_CONTROLLER_INIT_CONFIG_DEFAULT();
    ret = esp_bt_controller_init(&bt_cfg);
    if (ret)
    {
        ESP_LOGE(HID_DEMO_TAG, "%s initialize controller failed\n", __func__);
        return;
    }

    ret = esp_bt_controller_enable(ESP_BT_MODE_BLE);
    if (ret)
    {
        ESP_LOGE(HID_DEMO_TAG, "%s enable controller failed\n", __func__);
        return;
    }

    ret = esp_bluedroid_init();
    if (ret)
    {
        ESP_LOGE(HID_DEMO_TAG, "%s init bluedroid failed\n", __func__);
        return;
    }

    ret = esp_bluedroid_enable();
    if (ret)
    {
        ESP_LOGE(HID_DEMO_TAG, "%s init bluedroid failed\n", __func__);
        return;
    }

    if ((ret = esp_hidd_profile_init()) != ESP_OK)
    {
        ESP_LOGE(HID_DEMO_TAG, "%s init bluedroid failed\n", __func__);
    }

    ///register the callback function to the gap module
    esp_ble_gap_register_callback(gap_event_handler);
    esp_hidd_register_callbacks(hidd_event_callback);

    /* set the security iocap & auth_req & key size & init key response key parameters to the stack*/
    esp_ble_auth_req_t auth_req = ESP_LE_AUTH_BOND; //bonding with peer device after authentication
    esp_ble_io_cap_t iocap = ESP_IO_CAP_NONE;       //set the IO capability to No output No input
    uint8_t key_size = 16;                          //the key size should be 7~16 bytes
    uint8_t init_key = ESP_BLE_ENC_KEY_MASK | ESP_BLE_ID_KEY_MASK;
    uint8_t rsp_key = ESP_BLE_ENC_KEY_MASK | ESP_BLE_ID_KEY_MASK;
    esp_ble_gap_set_security_param(ESP_BLE_SM_AUTHEN_REQ_MODE, &auth_req, sizeof(uint8_t));
    esp_ble_gap_set_security_param(ESP_BLE_SM_IOCAP_MODE, &iocap, sizeof(uint8_t));
    esp_ble_gap_set_security_param(ESP_BLE_SM_MAX_KEY_SIZE, &key_size, sizeof(uint8_t));
    /* If your BLE device act as a Slave, the init_key means you hope which types of key of the master should distribute to you,
    and the response key means which key you can distribute to the Master;
    If your BLE device act as a master, the response key means you hope which types of key of the slave should distribute to you, 
    and the init key means which key you can distribute to the slave. */
    esp_ble_gap_set_security_param(ESP_BLE_SM_SET_INIT_KEY, &init_key, sizeof(uint8_t));
    esp_ble_gap_set_security_param(ESP_BLE_SM_SET_RSP_KEY, &rsp_key, sizeof(uint8_t));

    setupGpio();

    //xTaskCreate(&hid_demo_task, "hid_task", 2048, NULL, 5, NULL);

    //xTaskCreate(&task_test_SSD1306, "spi_task", 2048, NULL, 5, NULL);
    task_test_SSD1306();
}
