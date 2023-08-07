import Swiper from '/node_modules/swiper/swiper-bundle.min.mjs';

// 方法一；导出实例化构造函数参数。
let init = {
    init: false,
    direction: 'horizontal',
    autoplay: true,
    pagination: {
        el: '.swiper-pagination'
    },
    navigation: {
        nextEl: '.swiper-button-next',
        prevEl: '.swiper-button-prev'
    }
};

export { Swiper, init };
// 方法二；导出实例化函数，返回Swiper类型。
export function newSwiper() {
    return new Swiper ('.swiper', {
        init: false,
        direction: 'horizontal',
        autoplay: true,
        pagination: {
            el: '.swiper-pagination'
        },
        navigation: {
            nextEl: '.swiper-button-next',
            prevEl: '.swiper-button-prev'
        }
    });
}