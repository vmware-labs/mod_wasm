<?php

use Twig\Environment;
use Twig\Error\LoaderError;
use Twig\Error\RuntimeError;
use Twig\Extension\SandboxExtension;
use Twig\Markup;
use Twig\Sandbox\SecurityError;
use Twig\Sandbox\SecurityNotAllowedTagError;
use Twig\Sandbox\SecurityNotAllowedFilterError;
use Twig\Sandbox\SecurityNotAllowedFunctionError;
use Twig\Source;
use Twig\Template;

/* core/themes/olivero/templates/navigation/menu--primary-menu.html.twig */
class __TwigTemplate_3b9712f140e09e6847ec3e3441d3a650 extends Template
{
    private $source;
    private $macros = [];

    public function __construct(Environment $env)
    {
        parent::__construct($env);

        $this->source = $this->getSourceContext();

        $this->parent = false;

        $this->blocks = [
        ];
        $this->sandbox = $this->env->getExtension('\Twig\Extension\SandboxExtension');
        $this->checkSecurity();
    }

    protected function doDisplay(array $context, array $blocks = [])
    {
        $macros = $this->macros;
        // line 23
        echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->extensions['Drupal\Core\Template\TwigExtension']->attachLibrary("olivero/navigation-primary"), "html", null, true);
        echo "

";
        // line 25
        $macros["menus"] = $this->macros["menus"] = $this;
        // line 26
        echo "
";
        // line 31
        $context["attributes"] = twig_get_attribute($this->env, $this->source, ($context["attributes"] ?? null), "addClass", [0 => "menu"], "method", false, false, true, 31);
        // line 32
        echo $this->extensions['Drupal\Core\Template\TwigExtension']->renderVar(twig_call_macro($macros["menus"], "macro_menu_links", [($context["items"] ?? null), ($context["attributes"] ?? null), 0, "primary-menu-item-"], 32, $context, $this->getSourceContext()));
        echo "

";
    }

    // line 34
    public function macro_menu_links($__items__ = null, $__attributes__ = null, $__menu_level__ = null, $__aria_id__ = null, ...$__varargs__)
    {
        $macros = $this->macros;
        $context = $this->env->mergeGlobals([
            "items" => $__items__,
            "attributes" => $__attributes__,
            "menu_level" => $__menu_level__,
            "aria_id" => $__aria_id__,
            "varargs" => $__varargs__,
        ]);

        $blocks = [];

        ob_start(function () { return ''; });
        try {
            // line 35
            echo "  ";
            $context["primary_nav_level"] = ("primary-nav__menu--level-" . (($context["menu_level"] ?? null) + 1));
            // line 36
            echo "  ";
            $context["drupal_selector_primary_nav_level"] = (((($context["menu_level"] ?? null) <= 1)) ? (("primary-nav-menu--level-" . (($context["menu_level"] ?? null) + 1))) : (false));
            // line 37
            echo "  ";
            $context["is_top_level_menu"] = (($context["menu_level"] ?? null) == 0);
            // line 38
            echo "  ";
            $macros["menus"] = $this;
            // line 39
            echo "  ";
            if (($context["items"] ?? null)) {
                // line 40
                echo "
    ";
                // line 45
                echo "    ";
                if ((($context["menu_level"] ?? null) == 1)) {
                    // line 46
                    echo "      <span data-drupal-selector=\"primary-nav-menu-🥕\" class=\"primary-nav__menu-🥕\"></span>
    ";
                }
                // line 48
                echo "
    <ul ";
                // line 49
                echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(twig_get_attribute($this->env, $this->source, twig_get_attribute($this->env, $this->source, ($context["attributes"] ?? null), "addClass", [0 => "primary-nav__menu", 1 => ($context["primary_nav_level"] ?? null)], "method", false, false, true, 49), "setAttribute", [0 => "data-drupal-selector", 1 => ($context["drupal_selector_primary_nav_level"] ?? null)], "method", false, false, true, 49), 49, $this->source), "html", null, true);
                echo ">
      ";
                // line 50
                $context["attributes"] = twig_get_attribute($this->env, $this->source, ($context["attributes"] ?? null), "removeClass", [0 => ($context["primary_nav_level"] ?? null)], "method", false, false, true, 50);
                // line 51
                echo "      ";
                $context['_parent'] = $context;
                $context['_seq'] = twig_ensure_traversable(($context["items"] ?? null));
                $context['loop'] = [
                  'parent' => $context['_parent'],
                  'index0' => 0,
                  'index'  => 1,
                  'first'  => true,
                ];
                if (is_array($context['_seq']) || (is_object($context['_seq']) && $context['_seq'] instanceof \Countable)) {
                    $length = count($context['_seq']);
                    $context['loop']['revindex0'] = $length - 1;
                    $context['loop']['revindex'] = $length;
                    $context['loop']['length'] = $length;
                    $context['loop']['last'] = 1 === $length;
                }
                foreach ($context['_seq'] as $context["_key"] => $context["item"]) {
                    // line 52
                    echo "
        ";
                    // line 53
                    if ((twig_get_attribute($this->env, $this->source, twig_get_attribute($this->env, $this->source, $context["item"], "url", [], "any", false, false, true, 53), "isRouted", [], "any", false, false, true, 53) && (twig_get_attribute($this->env, $this->source, twig_get_attribute($this->env, $this->source, $context["item"], "url", [], "any", false, false, true, 53), "routeName", [], "any", false, false, true, 53) == "<nolink>"))) {
                        // line 54
                        echo "          ";
                        $context["menu_item_type"] = "nolink";
                        // line 55
                        echo "        ";
                    } elseif ((twig_get_attribute($this->env, $this->source, twig_get_attribute($this->env, $this->source, $context["item"], "url", [], "any", false, false, true, 55), "isRouted", [], "any", false, false, true, 55) && (twig_get_attribute($this->env, $this->source, twig_get_attribute($this->env, $this->source, $context["item"], "url", [], "any", false, false, true, 55), "routeName", [], "any", false, false, true, 55) == "<button>"))) {
                        // line 56
                        echo "          ";
                        $context["menu_item_type"] = "button";
                        // line 57
                        echo "        ";
                    } else {
                        // line 58
                        echo "          ";
                        $context["menu_item_type"] = "link";
                        // line 59
                        echo "        ";
                    }
                    // line 60
                    echo "
        ";
                    // line 61
                    $context["item_classes"] = [0 => "primary-nav__menu-item", 1 => ("primary-nav__menu-item--" . $this->sandbox->ensureToStringAllowed(                    // line 63
($context["menu_item_type"] ?? null), 63, $this->source)), 2 => ("primary-nav__menu-item--level-" . (                    // line 64
($context["menu_level"] ?? null) + 1)), 3 => ((twig_get_attribute($this->env, $this->source,                     // line 65
$context["item"], "in_active_trail", [], "any", false, false, true, 65)) ? ("primary-nav__menu-item--active-trail") : ("")), 4 => ((twig_get_attribute($this->env, $this->source,                     // line 66
$context["item"], "below", [], "any", false, false, true, 66)) ? ("primary-nav__menu-item--has-children") : (""))];
                    // line 69
                    echo "
        ";
                    // line 70
                    $context["link_classes"] = [0 => "primary-nav__menu-link", 1 => ("primary-nav__menu-link--" . $this->sandbox->ensureToStringAllowed(                    // line 72
($context["menu_item_type"] ?? null), 72, $this->source)), 2 => ("primary-nav__menu-link--level-" . (                    // line 73
($context["menu_level"] ?? null) + 1)), 3 => ((twig_get_attribute($this->env, $this->source,                     // line 74
$context["item"], "in_active_trail", [], "any", false, false, true, 74)) ? ("primary-nav__menu-link--active-trail") : ("")), 4 => ((twig_get_attribute($this->env, $this->source,                     // line 75
$context["item"], "below", [], "any", false, false, true, 75)) ? ("primary-nav__menu-link--has-children") : (""))];
                    // line 78
                    echo "
        <li";
                    // line 79
                    echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(twig_get_attribute($this->env, $this->source, twig_get_attribute($this->env, $this->source, twig_get_attribute($this->env, $this->source, $context["item"], "attributes", [], "any", false, false, true, 79), "addClass", [0 => ($context["item_classes"] ?? null)], "method", false, false, true, 79), "setAttribute", [0 => "data-drupal-selector", 1 => (((($context["is_top_level_menu"] ?? null) && twig_get_attribute($this->env, $this->source, $context["item"], "below", [], "any", false, false, true, 79))) ? ("primary-nav-menu-item-has-children") : (false))], "method", false, false, true, 79), 79, $this->source), "html", null, true);
                    echo ">
          ";
                    // line 85
                    echo "          ";
                    $context["aria_id"] = \Drupal\Component\Utility\Html::getId(($this->sandbox->ensureToStringAllowed(($context["aria_id"] ?? null), 85, $this->source) . $this->sandbox->ensureToStringAllowed(twig_get_attribute($this->env, $this->source, $context["loop"], "index", [], "any", false, false, true, 85), 85, $this->source)));
                    // line 86
                    echo "          ";
                    ob_start(function () { return ''; });
                    // line 87
                    echo "            <span class=\"primary-nav__menu-link-inner primary-nav__menu-link-inner--level-";
                    echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, (($context["menu_level"] ?? null) + 1), "html", null, true);
                    echo "\">";
                    echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(twig_get_attribute($this->env, $this->source, $context["item"], "title", [], "any", false, false, true, 87), 87, $this->source), "html", null, true);
                    echo "</span>
          ";
                    $context["link_title"] = ('' === $tmp = ob_get_clean()) ? '' : new Markup($tmp, $this->env->getCharset());
                    // line 89
                    echo "
          ";
                    // line 90
                    if (((($context["menu_item_type"] ?? null) == "link") || (($context["menu_item_type"] ?? null) == "nolink"))) {
                        // line 91
                        echo "            ";
                        echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->extensions['Drupal\Core\Template\TwigExtension']->getLink((((($context["menu_item_type"] ?? null) == "link")) ? (($context["link_title"] ?? null)) : (twig_get_attribute($this->env, $this->source, $context["item"], "title", [], "any", false, false, true, 91))), $this->sandbox->ensureToStringAllowed(twig_get_attribute($this->env, $this->source, $context["item"], "url", [], "any", false, false, true, 91), 91, $this->source), ["class" =>                         // line 92
($context["link_classes"] ?? null), "data-drupal-selector" => ((                        // line 93
($context["is_top_level_menu"] ?? null)) ? ("primary-nav-menu-link-has-children") : (false))]), "html", null, true);
                        // line 95
                        echo "

            ";
                        // line 97
                        if (twig_get_attribute($this->env, $this->source, $context["item"], "below", [], "any", false, false, true, 97)) {
                            // line 98
                            echo "              ";
                            // line 103
                            echo "              ";
                            if (($context["is_top_level_menu"] ?? null)) {
                                // line 104
                                echo "                ";
                                $context["toggle_button_attributes"] = $this->extensions['Drupal\Core\Template\TwigExtension']->createAttribute(["class" => "primary-nav__button-toggle", "data-drupal-selector" => "primary-nav-submenu-toggle-button", "aria-controls" =>                                 // line 107
($context["aria_id"] ?? null), "aria-expanded" => "false", "aria-hidden" => "true", "tabindex" => "-1"]);
                                // line 112
                                echo "
                <button";
                                // line 113
                                echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(($context["toggle_button_attributes"] ?? null), 113, $this->source), "html", null, true);
                                echo ">
                  <span class=\"visually-hidden\">";
                                // line 114
                                echo $this->extensions['Drupal\Core\Template\TwigExtension']->renderVar(t("@title sub-navigation", ["@title" => twig_get_attribute($this->env, $this->source, $context["item"], "title", [], "any", false, false, true, 114)]));
                                echo "</span>
                  <span class=\"icon--menu-toggle\"></span>
                </button>
              ";
                            }
                            // line 118
                            echo "
              ";
                            // line 119
                            $context["attributes"] = twig_get_attribute($this->env, $this->source, ($context["attributes"] ?? null), "setAttribute", [0 => "id", 1 => ($context["aria_id"] ?? null)], "method", false, false, true, 119);
                            // line 120
                            echo "              ";
                            echo $this->extensions['Drupal\Core\Template\TwigExtension']->renderVar(twig_call_macro($macros["menus"], "macro_menu_links", [twig_get_attribute($this->env, $this->source, $context["item"], "below", [], "any", false, false, true, 120), ($context["attributes"] ?? null), (($context["menu_level"] ?? null) + 1), ($context["aria_id"] ?? null)], 120, $context, $this->getSourceContext()));
                            echo "
            ";
                        }
                        // line 122
                        echo "
          ";
                    } elseif ((                    // line 123
($context["menu_item_type"] ?? null) == "button")) {
                        // line 124
                        echo "
            ";
                        // line 125
                        echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->extensions['Drupal\Core\Template\TwigExtension']->getLink($this->sandbox->ensureToStringAllowed(($context["link_title"] ?? null), 125, $this->source), $this->sandbox->ensureToStringAllowed(twig_get_attribute($this->env, $this->source, $context["item"], "url", [], "any", false, false, true, 125), 125, $this->source), ["class" =>                         // line 126
($context["link_classes"] ?? null), "aria-controls" => (((                        // line 127
($context["is_top_level_menu"] ?? null) && twig_get_attribute($this->env, $this->source, $context["item"], "below", [], "any", false, false, true, 127))) ? (($context["aria_id"] ?? null)) : (false)), "aria-expanded" => (((                        // line 128
($context["is_top_level_menu"] ?? null) && twig_get_attribute($this->env, $this->source, $context["item"], "below", [], "any", false, false, true, 128))) ? ("false") : (false)), "data-drupal-selector" => (((                        // line 129
($context["is_top_level_menu"] ?? null) && twig_get_attribute($this->env, $this->source, $context["item"], "below", [], "any", false, false, true, 129))) ? ("primary-nav-submenu-toggle-button") : (false))]), "html", null, true);
                        // line 130
                        echo "

            ";
                        // line 132
                        $context["attributes"] = twig_get_attribute($this->env, $this->source, ($context["attributes"] ?? null), "setAttribute", [0 => "id", 1 => ($context["aria_id"] ?? null)], "method", false, false, true, 132);
                        // line 133
                        echo "            ";
                        echo $this->extensions['Drupal\Core\Template\TwigExtension']->renderVar(twig_call_macro($macros["menus"], "macro_menu_links", [twig_get_attribute($this->env, $this->source, $context["item"], "below", [], "any", false, false, true, 133), ($context["attributes"] ?? null), (($context["menu_level"] ?? null) + 1), ($context["aria_id"] ?? null)], 133, $context, $this->getSourceContext()));
                        echo "
          ";
                    }
                    // line 135
                    echo "        </li>
      ";
                    ++$context['loop']['index0'];
                    ++$context['loop']['index'];
                    $context['loop']['first'] = false;
                    if (isset($context['loop']['length'])) {
                        --$context['loop']['revindex0'];
                        --$context['loop']['revindex'];
                        $context['loop']['last'] = 0 === $context['loop']['revindex0'];
                    }
                }
                $_parent = $context['_parent'];
                unset($context['_seq'], $context['_iterated'], $context['_key'], $context['item'], $context['_parent'], $context['loop']);
                $context = array_intersect_key($context, $_parent) + $_parent;
                // line 137
                echo "    </ul>
  ";
            }

            return ('' === $tmp = ob_get_contents()) ? '' : new Markup($tmp, $this->env->getCharset());
        } finally {
            ob_end_clean();
        }
    }

    public function getTemplateName()
    {
        return "core/themes/olivero/templates/navigation/menu--primary-menu.html.twig";
    }

    public function isTraitable()
    {
        return false;
    }

    public function getDebugInfo()
    {
        return array (  279 => 137,  264 => 135,  258 => 133,  256 => 132,  252 => 130,  250 => 129,  249 => 128,  248 => 127,  247 => 126,  246 => 125,  243 => 124,  241 => 123,  238 => 122,  232 => 120,  230 => 119,  227 => 118,  220 => 114,  216 => 113,  213 => 112,  211 => 107,  209 => 104,  206 => 103,  204 => 98,  202 => 97,  198 => 95,  196 => 93,  195 => 92,  193 => 91,  191 => 90,  188 => 89,  180 => 87,  177 => 86,  174 => 85,  170 => 79,  167 => 78,  165 => 75,  164 => 74,  163 => 73,  162 => 72,  161 => 70,  158 => 69,  156 => 66,  155 => 65,  154 => 64,  153 => 63,  152 => 61,  149 => 60,  146 => 59,  143 => 58,  140 => 57,  137 => 56,  134 => 55,  131 => 54,  129 => 53,  126 => 52,  108 => 51,  106 => 50,  102 => 49,  99 => 48,  95 => 46,  92 => 45,  89 => 40,  86 => 39,  83 => 38,  80 => 37,  77 => 36,  74 => 35,  58 => 34,  51 => 32,  49 => 31,  46 => 26,  44 => 25,  39 => 23,);
    }

    public function getSourceContext()
    {
        return new Source("", "core/themes/olivero/templates/navigation/menu--primary-menu.html.twig", "/usr/local/apache2/htdocs/drupal-10-zero/core/themes/olivero/templates/navigation/menu--primary-menu.html.twig");
    }
    
    public function checkSecurity()
    {
        static $tags = array("import" => 25, "set" => 31, "macro" => 34, "if" => 39, "for" => 51);
        static $filters = array("escape" => 23, "clean_id" => 85, "t" => 114);
        static $functions = array("attach_library" => 23, "link" => 91, "create_attribute" => 104);

        try {
            $this->sandbox->checkSecurity(
                ['import', 'set', 'macro', 'if', 'for'],
                ['escape', 'clean_id', 't'],
                ['attach_library', 'link', 'create_attribute']
            );
        } catch (SecurityError $e) {
            $e->setSourceContext($this->source);

            if ($e instanceof SecurityNotAllowedTagError && isset($tags[$e->getTagName()])) {
                $e->setTemplateLine($tags[$e->getTagName()]);
            } elseif ($e instanceof SecurityNotAllowedFilterError && isset($filters[$e->getFilterName()])) {
                $e->setTemplateLine($filters[$e->getFilterName()]);
            } elseif ($e instanceof SecurityNotAllowedFunctionError && isset($functions[$e->getFunctionName()])) {
                $e->setTemplateLine($functions[$e->getFunctionName()]);
            }

            throw $e;
        }

    }
}
