
asn-winit -> asn-wgpu -> [wgpu-handler]draw(wgpu_context)

                          Arc<Mutex<[wgpu-handler]>>update(delta_time)

[asn-winit] -> [asn-wgpu]
            -> [wgpu-handler]


// как заполнять gui-компоненты до вызова init ?
// State -> Loaded/Unloaded
// Option -> Option<Element>
// FnOnce(GraphContext) -> new TAsnGuiHandler()
// Для примера сделаем решение с Option<Element>

Бесплатную музыку и звуки можно взять отсюда:
https://zvukogram.com/zvuk/88634/
